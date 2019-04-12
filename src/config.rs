//use directories::{BaseDirs, UserDirs, ProjectDirs};
//use std::path::Path;
//use std::fs;
//use serde::{Serialize, Deserialize};
//use serde_json::Value;
//use serde_json::json;
//
//#[derive(Serialize, Deserialize)]
//pub struct Config {
//  workflow_locations: vec<String>,
//}
//
//impl Config {
//
//  fn new() -> Config {
//    Config {
//      workflow_locations: Vec!()
//    }
//  }
//
//}
//
//fn config(qualifier: &str, organisation: &str, application: &str) -> Option<Config> {
//
//  //attempt to load config
//  let configDirectory = ProjectDirs::from(qualifier, organisation, application);
//
//  configDirectory.map(|dirs| {
//    let configFile = Path::new(dirs.config_dir().with_file_name("config"));
//
//
//
//    if !configFile.exists() {
//
//      Config
//
//      fs::write(configFile, )
//    }
//
//    Config{
//      workflow_locations: Vec!()
//    }
//  })
//
//}