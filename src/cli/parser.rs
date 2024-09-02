use clap::Parser;

use super::args::CliArgs;

pub trait ArgsParser {
    fn parse_args() -> CliArgs;
}

pub struct CliArgsParser;

impl ArgsParser for CliArgsParser {
    fn parse_args() -> CliArgs {
        CliArgs::parse()
    }
}
