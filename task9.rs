// Дана переменная int64. Разработать программу которая устанавливает i-й бит в 1 или 0.
// +
fn one_bit(num: i64, pos: usize) -> i64 {
    num | (1 << pos)
}

fn zero_bit(num: i64, pos: usize) -> i64 {
    num & !(1 << pos)
}

fn main() {
    let num: i64 = 0b1011; // число
    let bitpos = 2; // позиция бита

    let new_num = one_bit(num, bitpos);
    println!(
        "Число после установки {}-го бита в 1: {:b}",
        bitpos, new_num
    );

    let new_num = zero_bit(num, bitpos);
    println!(
        "Число после установки {}-го бита в 0: {:b}",
        bitpos, new_num
    );
}
Пошаговая инструкция по выполнению лабораторной работы на Windows 10
Задание 1. Резервная копия реестра
1. Создайте копию реестра
Нажмите Win + R, введите regedit и нажмите Enter.
Откроется редактор реестра. В верхнем меню выберите Файл → Экспорт.
В окне экспорта:
Выберите место сохранения (например, C:\Backup\registry_backup.reg).
Укажите "Весь реестр".
Нажмите Сохранить.
2. Измените обои рабочего стола при помощи реестра
В редакторе реестра перейдите в:
mathematica
Copy
Edit
HKEY_CURRENT_USER\Control Panel\Desktop
Найдите параметр Wallpaper.
Дважды кликните по нему и укажите путь к новому изображению (например, C:\Users\Имя_Пользователя\Pictures\new_wallpaper.jpg).
Закройте редактор реестра.
Чтобы изменения вступили в силу, выполните:
Откройте командную строку (Win + R → cmd → Enter).
Введите команду:
Copy
Edit
RUNDLL32.EXE user32.dll,UpdatePerUserSystemParameters
Нажмите Enter.
3. Установите отображение версии Windows на рабочем столе
Перейдите в реестре:
mathematica
Copy
Edit
HKEY_CURRENT_USER\Control Panel\Desktop
Найдите параметр PaintDesktopVersion (если его нет, создайте DWORD (32-бит)).
Установите значение 1.
Закройте редактор и перезагрузите компьютер.
4. Восстановите настройки реестра
Откройте редактор реестра (Win + R → regedit → Enter).
В верхнем меню выберите Файл → Импорт.
Укажите путь к резервной копии (C:\Backup\registry_backup.reg).
Нажмите Открыть, дождитесь завершения восстановления.
Перезагрузите компьютер и убедитесь, что изменения отменены.
Задание 2. Настройка компьютера
1. Создайте точку восстановления системы
Нажмите Win + R, введите sysdm.cpl и нажмите Enter.
Перейдите во вкладку Защита системы.
Выберите диск C:\, нажмите Настроить, включите защиту.
Нажмите Создать, введите имя точки (например, Backup_Registry), нажмите Создать.
2. Настройка с помощью reg-файлов
Добавьте значок Корзины в "Мой компьютер"
Откройте Блокнот.
Вставьте код:
csharp
Copy
Edit
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{645FF040-5081-101B-9F08-00AA002F954E}]
Сохраните файл с расширением .reg, например, recycle_bin.reg.
Дважды кликните по файлу и подтвердите внесение изменений.
Добавьте "Редактор реестра" в контекстное меню "Мой компьютер"
Откройте Блокнот.
Вставьте:
csharp
Copy
Edit
Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\DesktopBackground\Shell\Registry]
@="Редактор реестра"
"Icon"="regedit.exe"

[HKEY_CLASSES_ROOT\DesktopBackground\Shell\Registry\Command]
@="regedit.exe"
Сохраните как add_regedit.reg, запустите.
Отключите стрелки ярлыков
Откройте Блокнот, вставьте:
csharp
Copy
Edit
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons]
"29"=" "
Сохраните как remove_shortcut_arrows.reg, запустите.
Перезагрузите компьютер.
Поменяйте раскладку клавиатуры в окне приветствия
Откройте Блокнот, вставьте:
arduino
Copy
Edit
Windows Registry Editor Version 5.00

[HKEY_USERS\.DEFAULT\Keyboard Layout\Preload]
"1"="00000419" ; Русская раскладка
Сохраните как change_keyboard.reg, запустите.
3. Создайте reg-файл для отмены изменений
Создайте restore.reg и добавьте обратные настройки (например, удалить значок "Корзины"):
css
Copy
Edit
Windows Registry Editor Version 5.00

[-HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{645FF040-5081-101B-9F08-00AA002F954E}]
Запустите restore.reg и перезагрузите компьютер.
Задание 3. Настройка автоматического входа в систему
1. Создайте пользователя
Откройте Параметры → Учетные записи → Семья и другие пользователи.
Нажмите Добавить пользователя.
Введите имя и пароль.
2. Настройте автоматический вход
Нажмите Win + R, введите netplwiz, нажмите Enter.
Выберите пользователя, уберите галочку "Требовать ввод пароля".
Введите пароль, нажмите OK.
3. Создайте систему учёта входов
Откройте редактор реестра, перейдите:
Copy
Edit
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsNT\CurrentVersion\Winlogon
Создайте параметр DWORD с именем LogonCount и значением (например, 10).
Создайте .bat-файл для уменьшения счетчика:
bash
Copy
Edit
@echo off
reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsNT\CurrentVersion\Winlogon" /v LogonCount /t REG_DWORD /d %count% /f
Добавьте .bat в автозагрузку (C:\Users\%USERNAME%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup).
Задание 4. Настройка безопасности
1. Создайте сетевую папку
Откройте проводник, выберите папку.
Нажмите Свойства → Доступ → Общий доступ.
Выберите "Все", установите права на чтение и запись.
2. Запретите неявный доступ
В командной строке (cmd от имени администратора) выполните:
bash
Copy
Edit
net config server /hidden:yes
Перезагрузите компьютер.
3. Проверьте доступность компьютера через ping
В другом ПК (в одной сети) откройте командную строку.
Выполните:
nginx
Copy
Edit
ping 192.168.X.X
(замените X.X на ваш IP ipconfig).
Если ping не проходит, откройте Параметры → Сеть и Интернет → Защитник брандмауэра и разрешите ICMP (входящие эхо-запросы).
Завершите работу → предъявите преподавателю → сделайте резервную копию перед удалением изменений.
