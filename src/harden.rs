use std::process::{self, exit};
use std::error::Error;
use inquire::{MultiSelect, list_option::ListOption, validator::Validation};

pub mod kernel;

#[derive(Debug)]
struct Config<'a> {
    components: Vec<& 'a str>,
}

impl<'a> Config<'a> {
    fn inquire(feat_components: Vec<&'a str>) -> Result<Config<'a>, Box<dyn Error>> {

        let validator = |a: &[ListOption<&&str>]| {
            if a.len() < 1 {
                return Ok(Validation::Invalid("Select at least one OS component.".into()));
            }
            Ok(Validation::Valid)
        };

        let ans = MultiSelect::new(
            "Select the os Components which are to be hardened:",
            feat_components)
            .with_validator(validator)
            .prompt().unwrap();

        Ok(Config{ components: ans})
    }
}

pub fn harden_os() {
    let feat_components = vec!["Kernel"];
    
    let config = Config::inquire(feat_components).unwrap_or_else(|err| {
        println!("Error in inquiring OS components: {}", err);
        process::exit(1);
    });


}