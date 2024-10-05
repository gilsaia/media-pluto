use clap::Parser;
use clap::Subcommand;

mod hard_link;
mod transcode;
mod utils;

use hard_link::hard_link_files;
use hard_link::HardLinkArgs;
use transcode::transcode_files;
use transcode::TranscodeArgs;

#[derive(Parser)]
#[command(name = "Media Pluto")]
#[command(version = "0.1")]
#[command(about="Basic Media Tools",long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Hard Link Files")]
    HardLink(HardLinkArgs),
    #[command(about = "Transcode Files with h.265.")]
    Transcode(TranscodeArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::HardLink(args) => {
            hard_link_files(args)?;
        }
        Commands::Transcode(args) => {
            transcode_files(args)?;
        }
    }
    Ok(())
}
