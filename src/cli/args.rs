use clap::{Parser, Subcommand};

/// Основная структура для аргументов командной строки
#[derive(Parser, Debug)]
#[command(name = "Pan", version = "1.0", about = "Описание приложения")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "sumcode", about = "Суммариризует код проекта в один .txt файл")]
    Sumcode {
        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        project_dir: String,

        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        output: String,
    }
}
