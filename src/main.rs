use brim::cli::{Cli, Parser, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Test) => test(),
        None => test()
    }
}

fn test() {
    println!("this is testing...")
}