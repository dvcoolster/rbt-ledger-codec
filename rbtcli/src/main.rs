use clap::{Parser, Subcommand, CommandFactory};

/// rbtzip – next-generation RBT compressor
#[derive(Parser)]
#[command(name = "rbtzip", version, about = "RBT-powered compression tool", author)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file to .rbtz
    Compress {
        /// Input file path
        input: String,
        /// Output .rbtz path (optional)
        output: Option<String>,
    },
    /// Extract .rbtz archive
    Extract {
        /// Input .rbtz file
        input: String,
        /// Output path (optional)
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Compress { input, output }) => {
            println!("[stub] compress {} -> {:?}", input, output);
        }
        Some(Commands::Extract { input, output }) => {
            println!("[stub] extract {} -> {:?}", input, output);
        }
        None => {
            // If no subcommand, print help
            Cli::command().print_help().unwrap();
            println!();
        }
    }
} 