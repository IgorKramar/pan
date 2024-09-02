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
    #[command(name = "sumcode", about = "Суммаризует код проекта в один .txt файл")]
    Sumcode(SumcodeArgs),
}

#[derive(Parser, Debug)]
pub struct SumcodeArgs {
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub project_dir: String,

    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub output: Option<String>,

    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub exclude: Vec<String>,

    #[arg(long, help = "Include the content of files with the specified extensions (e.g., --with-file-content rs,toml)")]
    pub with_file_content: Option<Vec<String>>, 
}

