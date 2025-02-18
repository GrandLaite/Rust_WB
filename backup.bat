REM ПРИМЕР СОЗДАНИЯ РЕЗЕРВНОЙ КОПИИ БАЗЫ ДАННЫХ POSTGRESQL
CLS
ECHO OFF
CHCP 1251

REM Установка переменных окружения (измените пути и логин/пароль при необходимости)
SET PGBIN="C:\Program Files\PostgreSQL\16\bin"
SET PGDATABASE=postgres
SET PGHOST=localhost
SET PGPORT=5432
SET PGUSER=postgres
SET PGPASSWORD=1

REM Смена диска и переход в папку из которой запущен bat-файл
%~d0
CD %~dp0

REM Формирование имени файла резервной копии и файла-отчета
SET DATETIME=%DATE:~6,4%-%DATE:~3,2%-%DATE:~0,2%_%TIME:~0,2%-%TIME:~3,2%-%TIME:~6,2%
SET DUMPFILE=%PGDATABASE%_%DATETIME%.backup
SET LOGFILE=%PGDATABASE%_%DATETIME%.log
SET DUMPPATH="Backup\%DUMPFILE%"
SET LOGPATH="Backup\%LOGFILE%"

REM Создание папки Backup, если она не существует
IF NOT EXIST Backup MD Backup

REM Создание резервной копии (формат custom, подробный вывод)
CALL %PGBIN%\pg_dump.exe --format=custom --verbose --file=%DUMPPATH% 2>%LOGPATH%

REM Анализ кода завершения
IF NOT %ERRORLEVEL%==0 GOTO Error
GOTO Successfull

:Error
DEL %DUMPPATH%
MSG * "Ошибка при создании резервной копии базы данных. Смотрите backup.log."
ECHO %DATETIME% Ошибки при создании резервной копии %DUMPFILE%. Смотрите отчет %LOGFILE%. >> backup.log
GOTO End

:Successfull
ECHO %DATETIME% Успешное создание резервной копии %DUMPFILE% >> backup.log
GOTO End

:End
