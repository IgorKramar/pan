pub mod sumcode;

use crate::cli::args::Commands;

pub fn execute_command(command: Commands) {
    match command {
        Commands::Sumcode(sumcode_args) => {
            sumcode::sumcode_command(&sumcode_args);
        }
    }
}
