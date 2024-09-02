mod cli;
mod commands;
mod filesystem;

use cli::parser::ArgsParser; // Импортируем трейт ArgsParser
use cli::CliArgsParser; // Импортируем структуру CliArgsParser
use commands::execute_command; // Импортируем функцию execute_command

fn main() {
    let args = CliArgsParser::parse_args(); // Используем метод parse_args из трейта ArgsParser
    execute_command(args.command);
}
