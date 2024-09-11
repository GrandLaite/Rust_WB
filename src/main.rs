// Импортируем всё необходимое
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_postgres::{Error as PgError, NoTls};
use tracing::{error, info};

// Определяем структуры со всеми необходимыми полями для работы с нашими данными заказов
// С моей точки зрения это является реализацией второго требования. Мы чётко определяем какие поля с какими типами данных должны поступать и если в .json файле значение поля не соответствует
// заданному типу или есть какие-то иные поля, которые мы здесь не указали, сервис не будет работать с этим файлом и вернёт ошибку
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Item {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}

// Тут у нас логика, описывающая работу кэша как одного из хранилищ наших данных
struct OrderStorage {
    orders: HashMap<String, Arc<Order>>,
}

impl OrderStorage {
    fn new() -> Self {
        OrderStorage {
            orders: HashMap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        let order_uid = order.order_uid.clone();
        self.orders.insert(order_uid, Arc::new(order));
    }

    fn get_order(&self, order_uid: &str) -> Option<Arc<Order>> {
        self.orders.get(order_uid).cloned()
    }
}

type SharedOrderStorage = Arc<RwLock<OrderStorage>>;

// Далее 3 нижние функции реализуют всё, что касается работы с базой данных, а именно инициализацию базы, сохранение заказов в базе и получение заказов из базы
async fn init_db() -> Result<tokio_postgres::Client, PgError> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1 dbname=wbdb", NoTls)
            .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Ошибка подключения к базе данных: {}", e);
        }
    });

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS orders (
                order_uid TEXT PRIMARY KEY,
                order_data JSONB NOT NULL
            )",
            &[],
        )
        .await?;

    Ok(client)
}

async fn save_order(client: &tokio_postgres::Client, order: &Order) -> Result<(), PgError> {
    let order_json = serde_json::to_value(order).unwrap();
    client
        .execute(
            "INSERT INTO orders (order_uid, order_data) VALUES ($1, $2) 
             ON CONFLICT (order_uid) DO NOTHING",
            &[&order.order_uid, &order_json],
        )
        .await?;
    Ok(())
}

async fn get_order_from_db(
    client: &tokio_postgres::Client,
    order_uid: &str,
) -> Result<Option<Order>, PgError> {
    let row = client
        .query_opt(
            "SELECT order_data FROM orders WHERE order_uid = $1",
            &[&order_uid],
        )
        .await?;

    if let Some(row) = row {
        let order_json: Value = row.get(0);
        let order: Order = serde_json::from_value(order_json).unwrap();
        Ok(Some(order))
    } else {
        Ok(None)
    }
}

// Реализация обработчиков для методов GET и POST
async fn get_order(
    Extension(state): Extension<SharedOrderStorage>,
    Extension(db_client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(order_uid): axum::extract::Path<String>,
) -> impl IntoResponse {
    let state_read = state.read().await;
    match state_read.get_order(&order_uid) {
        Some(order) => Json(order.as_ref().clone()).into_response(),
        None => {
            drop(state_read);
            match get_order_from_db(&db_client, &order_uid).await {
                Ok(Some(order)) => {
                    let mut state_write = state.write().await;
                    state_write.add_order(order.clone());
                    Json(order).into_response()
                }
                Ok(None) => (StatusCode::NOT_FOUND, "Заказ не найден").into_response(),
                Err(err) => {
                    error!("Не удалось получить заказ: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка базы данных").into_response()
                }
            }
        }
    }
}

async fn create_order(
    Extension(state): Extension<SharedOrderStorage>,
    Extension(db_client): Extension<Arc<tokio_postgres::Client>>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    let state_read = state.read().await;
    if state_read.get_order(&order.order_uid).is_some() {
        return (
            StatusCode::CONFLICT,
            "Заказ с таким order_uid уже существует",
        )
            .into_response();
    }
    drop(state_read);

    match save_order(&db_client, &order).await {
        Ok(_) => {
            let mut state_write = state.write().await;
            state_write.add_order(order);

            (StatusCode::CREATED, "Заказ создан!").into_response()
        }
        Err(err) => {
            error!("Не удалось сохранить заказ: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Не удалось сохранить заказ",
            )
                .into_response()
        }
    }
}

// Подключаемся к БД,настраиваем маршруты, запускаем локальный сервер

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let shared_state: SharedOrderStorage = Arc::new(RwLock::new(OrderStorage::new()));

    let db_client = match init_db().await {
        Ok(client) => Arc::new(client),
        Err(e) => {
            error!("Не удалось подключиться к базе данных: {}", e);
            return;
        }
    };

    let app = Router::new()
        .route("/orders/:order_uid", get(get_order))
        .route("/orders", post(create_order))
        .layer(Extension(shared_state))
        .layer(Extension(db_client));

    info!("Сервер запущен по адресу http://0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
