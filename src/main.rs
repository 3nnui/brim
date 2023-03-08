use clap::{Parser, Subcommand};

mod harden;

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(name = "Brim")]
#[command(author = "3nnui")]
#[command(version = "1.0")]
#[command(about = "
▀█████████▄     ▄████████  ▄█    ▄▄▄▄███▄▄▄▄  
  ███    ███   ███    ███ ███  ▄██▀▀▀███▀▀▀██▄
  ███    ███   ███    ███ ███▌ ███   ███   ███
 ▄███▄▄▄██▀   ▄███▄▄▄▄██▀ ███▌ ███   ███   ███
▀▀███▀▀▀██▄  ▀▀███▀▀▀▀▀   ███▌ ███   ███   ███
  ███    ██▄ ▀███████████ ███  ███   ███   ███
  ███    ███   ███    ███ ███  ███   ███   ███
▄█████████▀    ███    ███ █▀    ▀█   ███   █▀ 
               ███    ███                     
               
Fedora post install tool for some moderate hardening...")]

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

    match &cli.command {
        Some(Commands::Harden) => harden::init_hardening(),
        None => ()
    }
}