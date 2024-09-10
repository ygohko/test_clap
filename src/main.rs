use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct Args {
    /// Command you want to do
    #[command(subcommand)]
    command: Option<Commands>,
    /// Revision that you choose
    #[arg(short, long)]
    revision: Option<i32>,
    /// Path to a file or directory that you need
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Commit,
    Log,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    let command = match args.command.unwrap() {
        Commands::Commit => "commit",
        Commands::Log => "log",
    };
    println!("{}", command);
    if args.revision.is_some() {
        println!("{}", args.revision.unwrap());
    }
    if args.path.is_some() {
        println!("{}", args.path.unwrap());
    }    
}
