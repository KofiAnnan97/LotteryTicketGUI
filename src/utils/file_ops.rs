use std::fs::{self, File, write, read_to_string};
use std::path::{Path, PathBuf};
use serde_json::{json, Result, Value};

pub static DATA_DIR : &str = "data";
pub static PRESET_PATH : &str = "./data/presets.json";
pub static HISTORY_PATH : &str = "./data/ticket_history.json";

pub fn create_data_dir() {
    let data_dir = Path::new("./data");
    if !Path::is_dir(data_dir){
        let _ = fs::create_dir_all(data_dir);
    }
}

pub fn create_ticket_json(path_str: &str) {
    create_data_dir();
    File::create_new(path_str).expect("Could not create ticket history storage.");
    let setup = json!([]);
    let setup_str = serde_json::to_string_pretty(&setup);
    save_data(HISTORY_PATH.to_string(), setup_str.expect("Failed to initialize ticket history."));
}

pub fn create_preset_json(path_str: &str){
    create_data_dir();
    File::create_new(path_str).expect("Could not create preset storage.");
    let setup = json!({"default":"","presets":[]});
    let setup_str = serde_json::to_string_pretty(&setup).unwrap();
    save_data(PRESET_PATH.to_string(), setup_str);
}

pub fn get_preset_data() -> Result<Value> {
    let data = read_to_string(PRESET_PATH.to_string()).unwrap();
    Ok(serde_json::from_str(&data)?)
}

pub fn save_data(fp: String, data: String) {
    write(fp, data).expect("Data could not be saved.");
}