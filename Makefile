# Makefile для проекта на Rust

# Название проекта
PROJECT_NAME := my_cli_app

# Команда сборки проекта
build:
	cargo build --release

# Команда для запуска тестов
test:
	cargo test

# Команда для запуска с примером аргументов
run:
	cargo run -- sumcode --project-dir ./src --output result.txt --with-file-content rs,toml

# Команда для очистки артефактов сборки
clean:
	cargo clean

# Команда для сборки и установки проекта локально
install:
	cargo install --path .

# Команда для генерации документации
doc:
	cargo doc --open
