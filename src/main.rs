
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "grocery")]
#[command(about = "A CLI for automating weekly grocery shopping")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Favorites,

    Plan {
        #[arg(short, long, default_value_t = 80)]
        budget: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Favorites => {
            println!("Your favourite products will appear here.");
        }
        Commands::Plan { budget } => {
            println!("Generating shopping plan with £{budget} budget");
        }
    }
}
