use std::error::Error;
use inquire::{MultiSelect};

pub mod kernel;

#[derive(Debug)]
struct Config<'a> {
    components: Vec<& 'a str>,
}

impl<'a> Config<'a> {
    fn inquire(feat_components: Vec<&'a str>) -> Result<Config<'a>, Box<dyn Error>> {
        let ans = MultiSelect::new(
            "Select the os Components which are to be hardened:",
            feat_components)
            .prompt().unwrap();

       Ok(Config{ components: ans})
    }
}

pub fn harden_os() {
    let feat_components = vec!["Kernel"];
    let config = Config::inquire(feat_components);
    print!("Debug {:?}", config);
}