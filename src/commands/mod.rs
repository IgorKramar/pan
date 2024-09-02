pub mod sumcode;

use crate::cli::args::Commands;

pub fn execute_command(command: Commands) {
    match command {
        Commands::Sumcode { project_dir, output } => {
            sumcode::sumcode_command(project_dir, output);
        }
    }
}
