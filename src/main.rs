use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long)]
    pos: String,
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
    println!("{}", args.pos);
}
