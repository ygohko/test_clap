use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long)]
    revision: Option<i32>,
    #[arg(long)]
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
