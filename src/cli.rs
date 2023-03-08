use clap::{Subcommand};

pub use clap::Parser;


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
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test,
}