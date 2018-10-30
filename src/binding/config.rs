use std::fs::File;
use std::io::prelude::*;
use serde_yaml::from_str as from_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config{
    endpoint: String,
    log4rs_file: String,
    deliver_cycle: i64,
    poll_cycle: i64
}

impl Config {
    pub fn new() -> Self{
        match Config::from_file("config.yml"){
            Some(c) => c,
            None => {
                Config{
                    endpoint: "https://platform-api.newrelic.com/platform/v1/metrics".into(),
                    log4rs_file: "log4rs.yml".into(),
                    deliver_cycle: 60,
                    poll_cycle: 20
                }
            }
        }
    }

    pub fn from_file(path: &str) -> Option<Self>{
        let f = File::open(path);
        if f.is_err(){
            let err = f.err();
            error!(target: "agent", "Could not open config file. Error: {:?}", err.as_ref());
            return None;
        }
        let mut config_string = String::new();
        match f.unwrap().read_to_string(&mut config_string){
            Ok(_) => {
                match from_yaml(&config_string){
                    Ok(c) => Some(c),
                    Err(e) => {
                        error!(target: "agent", "Config file couldn't be parsed. Error: {:?}", e);
                        None
                    }
                }
            },
            Err(e) => {
                error!(target: "agent", "Could not read config file. Error: {:?}", e);
                None          
            }
        }
    }

    pub fn deliver_cycle(&self) -> i64{
        self.deliver_cycle
    }

    pub fn poll_cycle(&self) -> i64{
        self.poll_cycle
    }

    pub fn get_endpoint(&self) -> String{
        self.endpoint.to_string()
    }

    pub fn log4rs_file(&self) -> String{
        self.log4rs_file.to_string()
    }
}
