use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pos: String,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    println!("{}", args.pos);
}
