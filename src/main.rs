use std::process;
use clap::{Parser, Subcommand};

mod harden;

const BANNER: &str = "
▀█████████▄     ▄████████  ▄█    ▄▄▄▄███▄▄▄▄  
  ███    ███   ███    ███ ███  ▄██▀▀▀███▀▀▀██▄
  ███    ███   ███    ███ ███▌ ███   ███   ███
 ▄███▄▄▄██▀   ▄███▄▄▄▄██▀ ███▌ ███   ███   ███
▀▀███▀▀▀██▄  ▀▀███▀▀▀▀▀   ███▌ ███   ███   ███
  ███    ██▄ ▀███████████ ███  ███   ███   ███
  ███    ███   ███    ███ ███  ███   ███   ███
▄█████████▀    ███    ███ █▀    ▀█   ███   █▀ 
               ███    ███                     
               
A fedora post install tool for some moderate hardening...";

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(name = "Brim")]
#[command(author = "3nnui")]
#[command(version = "1.0")]
#[command(about = BANNER)]

struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]

pub enum Commands {
    /// guided step by step os hardening
    Harden,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", BANNER);

    match &cli.command {
        Some(Commands::Harden) => harden::harden_os(),
        None => ()
    }
}