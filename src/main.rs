use clap::Parser;
use clap::Subcommand;

mod hard_link;
use hard_link::hard_link_files;
use hard_link::HardLinkArgs;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::HardLink(args) => {
            hard_link_files(args)?;
        }
    }
    Ok(())
}
