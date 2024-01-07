use clap::Parser;
use gen_color::{gradient, Cli, Subcommands, random};
use miette::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Subcommands::Gradient(options) => {
            gradient::generate(options)
                .wrap_err("when generating gradient")
        }
        Subcommands::Random(options) => {
            random::generate(options)
                .wrap_err("when generating random")
        }
    }
}
