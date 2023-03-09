use std::error::Error;
use std::process;
use inquire::{MultiSelect, InquireError};

pub mod kernel;

const featured_components: Vec<&str> = vec!["Kernel"];

pub struct Config<'a> {
    components: Vec<& 'a str>,
}

impl<'a> Config<'a> {
    pub fn inquire() -> Result<Config<'a>, Box<dyn Error>> {
        let ans = MultiSelect::new(
            "Select the os Components which are to be hardened:",
            featured_components)
            .prompt().unwrap_or_else(|err: InquireError| {
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            });

       Ok(Config{ components: ans})
    }
}