use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{Read, Write};
use std::{fs, io};

/// App configuration, serialized as json in $XDG_CONFIG_HOME/gpto.cfg
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Config {
    /// The OpenAI Api token
    pub token: String,
    /// Path to config file
    pub path: String,
}

impl Config {
    pub fn new(token: &str) -> Result<Config, String> {
        Ok(Config {
            path: generate_path()?,
            token: String::from(token),
        })
    }

    pub fn create(self) -> Result<Config, String> {
        let json = json!(self).to_string();
        let mut file = fs::File::create(&self.path).or(Err("Could not create file"))?;
        file.write_all(json.as_bytes())
            .or(Err("Could not write to file"))?;
        println!("Config successfully created in {}", &self.path);
        Ok(self)
    }

    pub fn load(path: &str) -> Result<Config, String> {
        let mut json = String::new();

        fs::File::open(path)
            .or(Err("Could not find file"))?
            .read_to_string(&mut json)
            .or(Err("Could not read to string"))?;

        serde_json::from_str::<Config>(&json).map_err(|_| String::from("Could not parse JSON"))
    }
}

pub fn get_or_create(config_path: Option<&str>) -> Result<Config, String> {
    let path: String = match config_path {
        None => generate_path()?,
        Some(path) => String::from(path).trim().to_owned(),
    };
    let desc = "Please enter your OpenAI API token from https://beta.openai.com/account/api-keys";

    match fs::File::open(&path) {
        Ok(_) => Config::load(&path),
        Err(_) => {
            let token = get_input(desc)?;
            Config::new(&token)?.create()
        }
    }
}

pub fn generate_path() -> Result<String, String> {
    let filename = if cfg!(test) { "test" } else { "gpto.cfg" };

    let config_directory = dirs::config_dir()
        .ok_or_else(|| String::from("Could not find config directory"))?
        .to_str()
        .ok_or_else(|| String::from("Could not convert config directory to string"))?
        .to_owned();
    Ok(format!("{}/{}", config_directory, filename))
}

pub fn get_input(desc: &str) -> Result<String, String> {
    if cfg!(test) {
        return Ok(String::from("5"));
    }

    let mut input = String::new();
    println!("{}", desc);
    io::stdin()
        .read_line(&mut input)
        .or(Err("error: unable to read user input"))?;

    Ok(String::from(input.trim()))
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_should_generate_config() {
        let config = Config::new("something").unwrap();
        assert_eq!(config.token, String::from("something"));
    }

    #[test]
    fn config_tests() {
        // These need to be run sequentially as they write to the filesystem.

        // Save and load
        // Build path
        let config_directory = dirs::config_dir().expect("could not get home directory");
        let config_directory_str = config_directory
            .to_str()
            .expect("could not set home directory to str");
        let path = format!("{}/test", config_directory_str);

        // Just in case there is a leftover config from a previous test run
        let _ = fs::remove_file(&path);

        // create and load
        let new_config = Config::new("faketoken").unwrap();
        let created_config = new_config.clone().create().unwrap();
        assert_eq!(new_config, created_config);
        let loaded_config = Config::load(&path).unwrap();
        assert_eq!(created_config, loaded_config);

        // get_or_create (create)
        let config = get_or_create(None);
        assert_eq!(
            config.clone(),
            Ok(Config {
                token: String::from("faketoken"),
                path: generate_path().unwrap(),
            })
        );
        delete_config(&path);

        // get_or_create (load)
        Config::new("alreadycreated").unwrap().create().unwrap();
        let config = get_or_create(None);
        assert_eq!(
            config.clone(),
            Ok(Config {
                token: String::from("alreadycreated"),
                path: generate_path().unwrap(),
            })
        );
        delete_config(&path);
    }

    #[test]
    fn custom_config_path() {
        let path = String::from("./tests/gpto.cfg");
        let loaded_config = Config::load(&path).unwrap();

        let config = Config {
            token: String::from("23984719029"),
            path: String::from("/home/vardy/dev/gpto/tests/gpto.cfg"),
        };
        assert_eq!(loaded_config, config);
    }

    fn delete_config(path: &str) {
        fs::remove_file(path).unwrap();
    }
}
