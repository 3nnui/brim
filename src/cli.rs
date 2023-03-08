use clap::{Subcommand};

pub use clap::Parser;

#[derive(Parser)]
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
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}