// Реализовать паттерн «адаптер» на любом примере.
// +

// Не использую реализацию отдельно старого метода для демонстрации его работы, поэтому вылезают предупреждения, поэтому я кидаю их в мут :)
#[allow(dead_code)]
struct OldLink;

#[allow(dead_code)]
impl OldLink {
    fn request_old_link(&self) {
        println!("Вы переходите по ссылке: tech.wildberriesshop.ru");
        println!("Упс...Не можем получить доступ к сайту.");
    }
}

struct NewLink;

impl NewLink {
    fn request_new_link(&self) {
        println!("Вы переходите по ссылке: tech.wildberries.ru");
        println!("Добро пожаловать!")
    }
}

struct LinkAdapter {
    new_link: NewLink,
}

impl LinkAdapter {
    fn new(new_link: NewLink) -> LinkAdapter {
        LinkAdapter { new_link }
    }

    fn request_old_link(&self) {
        self.new_link.request_new_link();
    }
}

fn main() {
    let adapter = LinkAdapter::new(NewLink);
    adapter.request_old_link();
}
