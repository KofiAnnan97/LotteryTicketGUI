use std::fs;
use std::time::SystemTime;
use std::vec;
use std::time;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Error, json};
use sqlx::types::Json;
use std::fs::{File, write, read_to_string};
use std::path::Path;
use chrono::offset::Utc;
use chrono::DateTime;

static PRESET_PATH : &str = "./data/presets.json";
static HISTORY_PATH : &str = "./data/ticket_history.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct Ticket {
    ticket_id: u32,
    pub preset_id: u32,
    pub created_time : String,
    pub created_date : String,
    pub numbers: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Presets {
    default: String,
    presets: Vec<TicketPreset>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TicketPreset {
    preset_id: u32,
    pub name: String,
    pub limits: String,
    pub created_time : String,
    pub created_date : String,
    pub modified_time: String,
    pub modified_data: String,
}

// Helper Functions

fn get_date_str() -> String {
    let now = SystemTime::now();
    let dt: DateTime<Utc> = now.into();
    dt.format("%d/%m/%Y").to_string()
}

fn get_time_str() -> String {
    let now = SystemTime::now();
    let dt: DateTime<Utc> = now.into();
    dt.format("%T").to_string()
}

fn convert_vec_to_str(nums: Vec<i32>) -> String {
    let mut temp : String = String::from("");
    for n in 0..nums.len()-1{
        temp += &format!("{},", nums[n].to_string());
    }
    temp += &nums[nums.len()-1].to_string();
    temp
}

fn convert_str_to_vec(nums_str: &String) -> Vec<i32> {
    let mut temp : Vec<i32> = Vec::new();
    let segments = (&nums_str).split(",");
    for part in segments{
        match part.parse::<i32>(){
            Ok(data) => temp.push(data),
            Err(e) => eprintln!("Error: {}", e)
        };
    }
    temp
}

fn create_data_dir() {
    let data_dir = Path::new("./data");
    if !Path::is_dir(data_dir){
        let _ = fs::create_dir_all(data_dir);
    }
}

fn create_ticket_json(path_str: &str) {
    create_data_dir();
    File::create_new(path_str).expect("Could not create ticket history storage.");
    let setup = json!([]);
    let setup_str = serde_json::to_string(&setup);
    save_data(HISTORY_PATH.to_string(), setup_str.expect("Failed to initialize ticket history."));
}

fn create_preset_json(path_str: &str){
    create_data_dir();
    File::create_new(path_str).expect("Could not create preset storage.");
    let setup = json!({"default":"","presets":[]});
    let setup_str = serde_json::to_string(&setup).unwrap();
    save_data(PRESET_PATH.to_string(), setup_str);
}

fn get_preset_data() -> Result<Value> {
    let data = read_to_string(PRESET_PATH.to_string()).unwrap();
    Ok(serde_json::from_str(&data)?)
}

fn is_preset_used(preset_id: u32) -> bool {
    let mut is_used : bool = false;
    match get_tickets() {
        Ok(tickets) => {
            for t in tickets.iter() {
                if t.preset_id == preset_id {
                    is_used = true;
                    break;
                }
            }
        }, 
        Err(e) => eprintln!("Error: {}", e)
    }
    is_used
}

fn update_tickets_preset_id(curr_id: u32, new_id: u32) {
    match get_tickets() {
        Ok(mut tickets) => {
            for t in tickets.iter_mut() {
                if t.preset_id == curr_id { t.preset_id = new_id; }
            }
            let tickets_str = serde_json::to_string(&tickets).expect("Could not update ticket preset id");
            save_data(HISTORY_PATH.to_string(), tickets_str);
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn update_presets_helper(presets: Vec<TicketPreset>) {
    match get_preset_data() {
        Ok(mut data) => {
            *data.get_mut("presets").unwrap() = json!(presets);
            let data_str = serde_json::to_string(&data).expect("Could not update presets");
            save_data(PRESET_PATH.to_string(), data_str);
        },
        Err(e) => eprintln!("Error: {}", e)
    };
}

fn save_data(fp: String, data: String) {
    write(fp, data).expect("Data could not be saved.");
}

// CREATE

pub fn add_ticket(p_id: u32, nums: Vec<i32>) -> Option<bool>{
    let path = Path::new(HISTORY_PATH);
    if !path.is_file(){ create_ticket_json(HISTORY_PATH); }
    match get_tickets(){
        Ok(mut tickets) => {
            let idx : u32 = tickets.len() as u32;
            let nums_str : String = convert_vec_to_str(nums);
            tickets.push(Ticket {
                ticket_id: idx,
                preset_id: p_id,
                created_date: get_date_str(),
                created_time: get_time_str(),
                numbers: nums_str
            });
            let data = serde_json::to_string(&tickets).unwrap();
            save_data(HISTORY_PATH.to_string(), data);
            return Ok::<bool, Error>(true).ok();
        },
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}

pub fn add_preset( name: &str, limits: Vec<i32>) -> Option<bool>{
    let path = Path::new(PRESET_PATH);
    if !path.is_file(){ create_preset_json(PRESET_PATH); }
    match get_presets() {
        Ok(mut presets) => {
            let mut unique : bool = true;
            for p in presets.iter() {
                if &p.name == name { 
                    unique = false;
                    break; 
                }
            }
            if unique {
                let limit_str : String = convert_vec_to_str(limits);
                let date = get_date_str();
                let time = get_time_str();
                presets.push(TicketPreset{
                    preset_id: presets.len() as u32,
                    name: name.to_string(),
                    limits: limit_str,
                    created_date: date.clone(),
                    created_time: time.clone(),
                    modified_data: date.clone(),
                    modified_time: time.clone()
                });
                if presets.len().clone() == 1 { set_default_preset(name.to_string()); }
                update_presets_helper(presets);
                return Ok::<bool, Error>(true).ok();
            }
            else{
                eprintln!("Duplicate name: {}", name);
            }
        },
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}

// READ

pub fn get_tickets() -> Result<Vec<Ticket>> {
    let data = read_to_string(HISTORY_PATH).unwrap();
    serde_json::from_str::<Vec<Ticket>>(&data)
}

pub fn get_ticket(ticket_id: u32) -> Option<Ticket> {
    match get_tickets() {
        Ok(tickets) => {
            if ticket_id < tickets.len() as u32{
                let idx = ticket_id as usize;
                let curr_ticket = &tickets[idx];
                return Ok::<Ticket, Error>(Ticket { 
                    ticket_id: curr_ticket.ticket_id, 
                    preset_id: curr_ticket.preset_id, 
                    created_time: curr_ticket.created_time.clone(), 
                    created_date: curr_ticket.created_date.clone(), 
                    numbers: curr_ticket.numbers.clone(), 
                }).ok();
            }
        },  
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}

pub fn get_presets() -> Result<Vec<TicketPreset>> {
    let data = read_to_string(PRESET_PATH).unwrap();
    let body : Value = serde_json::from_str(&data).expect("Could not parse data while getting presets");
    let presets = serde_json::to_string(&body["presets"]).unwrap();
    serde_json::from_str::<Vec<TicketPreset>>(&presets)
}

pub fn get_preset(preset_id: u32) -> Option<TicketPreset> {
    match get_presets() {
        Ok(presets) => {
            for p in presets {
                if p.preset_id == preset_id {
                    return Ok::<TicketPreset, Error>(p).ok();
                }
            }
        }, 
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}

pub fn get_default() -> Option<String> {
    let data = read_to_string(PRESET_PATH).unwrap();
    let body : Value = serde_json::from_str(&data).expect("Could not parse data while getting default.");
    //serde_json::to_string(&body["default"])
    let default_str = serde_json::to_string(&body["default"]).unwrap().to_string();
    let d_str = default_str[1..default_str.len()-1].to_string();
    return Ok::<String, Error>(d_str).ok();
    
}

pub fn get_preset_id(preset_name: String) -> Option<u32> {
    match get_presets(){
        Ok(presets) => {
            for p in presets.iter() {
                if *p.name == preset_name {
                    return Ok::<u32, Error>(p.preset_id).ok();
                }
            }
        }, 
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}

pub fn get_preset_limits(preset_id: u32) -> Option<Vec<i32>> {
    match get_presets() {
        Ok(presets) => {
            for p in presets.iter() {
                if p.preset_id == preset_id {
                    let data = convert_str_to_vec(&p.limits);
                    return Ok::<Vec<i32>, Error>(data).ok();
                }
            }
        }, 
        Err(e) => eprintln!("Error: {}", e)
    }
    None
}


// UPDATE

pub fn set_default_preset(name: String) {
    match get_preset_data() {
        Ok(mut data) => {
            *data.get_mut("default").unwrap() = json!(name);
            let data_str = serde_json::to_string(&data).expect("Could not update default preset");
            save_data(PRESET_PATH.to_string(), data_str);
        },
        Err(e) => eprintln!("Error: {}", e)
    };
}

fn update_preset_name(preset_name:String, new_name: String) {
    match get_presets() {
        Ok(mut presets) => {
            let mut is_present: bool = false;
            for p in presets.iter_mut(){
                if p.name == preset_name { 
                    p.name = new_name.clone();  
                    p.modified_data = get_date_str();
                    p.modified_time = get_time_str(); 
                    is_present = true;                 
                    /*match get_default(){
                        Ok(default_name) => {
                            if default_name == p.name {
                                let new_defualt = new_name.clone();
                                
                            }
                        }, 
                        Err(e) => eprintln!("Error: {}", e)
                    }*/
                    break; 
                }
            }
            if is_present { update_presets_helper(presets); }
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub fn update_preset_limits(id: u32, new_limits: Vec<i32>) {
    match get_presets() {
        Ok(mut presets) => {
            let mut is_present: bool = false;
            for p in presets.iter_mut(){
                if p.preset_id == id { 
                    let new_limits_str = convert_vec_to_str(new_limits);
                    p.limits = new_limits_str;  
                    p.modified_data = get_date_str();
                    p.modified_time = get_time_str(); 
                    is_present = true;                 
                    break; 
                }
            }
            if is_present { update_presets_helper(presets); }
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

// DELETE

pub fn delete_ticket(ticket_id: u32) -> bool {
    let mut is_ticket_deleted = false;
    match get_tickets() {
        Ok(mut tickets) => {
            let mut r_idx : usize = 0;
            let mut can_remove : bool = false;
            for i in 0..tickets.len() {
                if tickets[i].ticket_id == ticket_id {
                    r_idx = i;
                    can_remove = true;
                    break;
                }
            }
            if can_remove { 
                tickets.remove(r_idx);
                let mut idx : u32 = r_idx as u32;
                for j in r_idx..tickets.len() {
                    tickets[j].ticket_id = idx;
                    idx+=1;
                }
                let data_str = serde_json::to_string(&tickets).unwrap();
                save_data(HISTORY_PATH.to_string(), data_str);
                is_ticket_deleted = true;
            }
        },
        Err(e) => println!("Error: {}", e)
    }
    is_ticket_deleted
}

pub fn delete_all_tickets_by_preset_id(preset_id: u32) {
    match get_tickets() {
        Ok(mut tickets) => {
            let mut idx = 0;
            let mut new_tickets : Vec<Ticket> = Vec::new();
            for mut t in tickets{
                if t.preset_id != preset_id {
                    if idx != t.ticket_id { t.ticket_id = idx; }
                    new_tickets.push(t);
                    idx += 1;
                }
            }
            /*println!("{:?}", tickets);
            for i in tickets.len()..0 {
                println!("{}", i);
                if tickets[i].preset_id == preset_id {
                    tickets.remove(i);
                }
            }*/
            let data_str = serde_json::to_string(&new_tickets).unwrap();
            save_data(HISTORY_PATH.to_string(), data_str);
            
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

pub fn delete_preset(preset_id: u32) {
    match get_presets() {
        Ok(mut presets) => {
            let mut r_idx : usize = 0;
            let mut can_remove : bool = false;
            for i in 0..presets.len(){
                if presets[i].preset_id == preset_id {
                    r_idx = i;
                    if !is_preset_used(presets[i].preset_id) { can_remove = true; }
                    break;
                }
            }
            if can_remove {
                presets.remove(r_idx);
                let mut idx: u32 = r_idx as u32;
                for j in r_idx..presets.len() {
                    println!("{} {}", presets[j].preset_id, idx);
                    // update all tickets with new preset id
                    update_tickets_preset_id(presets[j].preset_id, idx);
                    // update preset with new id
                    presets[j].preset_id = idx;
                    idx += 1;
                }
                update_presets_helper(presets);
            }
            else {
                println!("Cannot delete preset \"{}\" because tickets are using it.", presets[r_idx].name);
            }
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(limits: Vec<i32>) -> Vec<i32> {
        let mut numbers : Vec<i32> = Vec::new();
        for i in 0..limits.len(){
            numbers.push(rand::random_range(1..limits[i]));
        }
        return numbers;
    }

    fn init () {
        // Add presets
        add_preset("TestLotto", vec![10,20,30,40,50]);
        add_preset("EzMoney", vec![70,70,70,25]);
        add_preset("DeleteMeHard", vec![2,2,2,2,2,2]);

        // Add tickets
        match get_preset_id("TestLotto".to_string()){
            Some(preset_id) => {
                for _i in 0..5 {
                    let limits = get_preset_limits(preset_id).unwrap();
                    let nums = generate(limits);
                    add_ticket(preset_id, nums);
                }
            },
            None => assert!(false)
        };
        match get_preset_id("EzMoney".to_string()) {
            Some(preset_id) => {
                for _i in 0..7 {
                    let limits = get_preset_limits(preset_id).unwrap();
                    let nums = generate(limits);
                    add_ticket(preset_id, nums);
                }
            },
            None => assert!(false)
        };
        match get_preset_id("DeleteMeHard".to_string()) {
            Some(preset_id) => {
                let limits = get_preset_limits(preset_id).unwrap();
                let nums = generate(limits);
                add_ticket(preset_id, nums);
            },
            None => assert!(false)
        };
    }

    fn clear() {
        // Clear Tickets
        if Path::new(HISTORY_PATH).is_file() {
            let empty_list = json!([]);
            let empty_list_str = serde_json::to_string(&empty_list).unwrap();
            save_data(HISTORY_PATH.to_string(), empty_list_str);
        }
        
        // Clear Presets
        if Path::new(PRESET_PATH).is_file() {    
            match get_preset_data(){
                Ok(mut data) => {
                    *data.get_mut("default").unwrap() = json!("");
                    *data.get_mut("presets").unwrap() = json!([]);
                    let data_str = serde_json::to_string(&data).unwrap();
                    save_data(PRESET_PATH.to_string(), data_str);
                }, 
                Err(e) => eprintln!("Error: {}", e)
            };
        }
        
    }

    #[test]
    fn add_presets_and_tickets_test() {
        clear();
        init();
    }

    #[test]
    fn update_preset_name_test() {
        clear();
        init();
        let old_name = "TestLotto".to_owned();
        let new_name = String::from("TestLOTTO");
        update_preset_name(old_name.clone(), new_name.clone());
        match get_preset_id(new_name.clone()) {
            Some(preset_id) => {
                let preset = get_preset(preset_id).unwrap();
                match get_default() {
                    Some(default_name) => {
                        let d_str = default_name.to_string();
                        if default_name == old_name {
                            set_default_preset(new_name.clone());
                        }
                        assert_eq!(new_name, preset.name)
                    },
                    None => assert!(false)
                }
            },
            None => assert!(false)
        }
    }

    #[test]
    fn update_preset_limits_test() {
        clear();
        init();
        let new_limits = vec![10, 10, 10, 10, 10, 10];
        match get_preset_id("DeleteMeHard".to_string()) {
            Some(preset_id) => {
                match get_preset(preset_id) {
                    Some (preset) => {
                        let new_limits_str = convert_vec_to_str(new_limits.clone());
                        assert_ne!(new_limits_str, preset.limits);
                    },
                    None => assert!(false)
                }
                update_preset_limits(preset_id, new_limits.clone());
                match get_preset(preset_id) {
                    Some (preset) => {
                        let new_limits_str = convert_vec_to_str(new_limits);
                        assert_eq!(new_limits_str, preset.limits)
                    },
                    None => assert!(false)
                }
            },
            None => assert!(false)
        }
    }

    #[test]
    fn delete_ticket_test() {
        clear();
        init();
        let first_ticket = get_ticket(0).unwrap();
        println!("First Ticket Entry: {:?}", first_ticket);
        let first_nums = first_ticket.numbers.clone();
        delete_ticket(0);
        let new_first_ticket = get_ticket(0).unwrap();
        println!("New First Ticket Entry: {:?}", new_first_ticket);
        assert_ne!(first_nums, new_first_ticket.numbers);
    }

    #[test]
    fn delete_all_tickets_for_preset_test() {
        clear();
        init();
        let deleted_id = get_preset_id("TestLotto".to_string()).unwrap();
        delete_all_tickets_by_preset_id(deleted_id);
        match get_tickets() {
            Ok(tickets) => {
                let mut no_tickets_with_preset : bool = true;
                for ticket in tickets {
                    if ticket.preset_id == deleted_id {
                        no_tickets_with_preset = false;
                        break;
                    }
                }
                assert!(no_tickets_with_preset);
            },
            Err(e) => assert!(false)
        }
    }

    #[test]
    fn delete_unused_preset() {
        clear();
        init();
        let preset_name = "DeleteMeEasy";
        add_preset(preset_name, vec![20, 20, 20]);
        match get_presets() {
            Ok(presets) => {
                let mut preset_exists : bool = false;
                for p in presets {
                    if p.name == preset_name {
                        preset_exists = true;
                        break;
                    }
                }
                assert!(preset_exists);
            },
            Err(e) => assert!(false)
        }
        let deleted_id = get_preset_id(preset_name.to_string()).unwrap();
        delete_preset(deleted_id);
        match get_presets() {
            Ok(presets) => {
                let mut preset_deleted : bool = true;
                for p in presets {
                    if p.name == preset_name {
                        preset_deleted = false;
                        break;
                    }
                }
                assert!(preset_deleted);
            },
            Err(e) => assert!(false)
        }
    }

    #[test]
    fn fail_to_delete_used_preset(){
        clear();
        init();
        let preset_name = "DeleteMeHard";
        let deleted_id = get_preset_id(preset_name.to_string()).unwrap();
        delete_preset(deleted_id);
        match get_presets() {
            Ok(presets) => {
                let mut not_deleted = false;
                for p in presets {
                    if p.name == preset_name {
                        not_deleted = true;
                        break;
                    }
                }
                assert!(not_deleted);
            }, 
            Err(e) => assert!(false)
        }
        /*delete_all_tickets_by_preset_id(deleted_id);
        match get_presets() {
            Ok(presets) => {
                let mut not_deleted = true;
                for p in presets {
                    if p.name == preset_name {
                        not_deleted = false;
                        break;
                    }
                }
                assert!(not_deleted);
            }, 
            Err(e) => assert!(false)
        }*/
    }

    #[test]
    fn reorder_presets_after_delete_test(){
        clear();
        let preset_name = "ForceReorder";
        add_preset(preset_name, vec![99, 99, 99, 99, 99]);
        init();
        let preset_id = get_preset_id(preset_name.to_string()).unwrap();
        assert_eq!(0, preset_id);
        delete_preset(preset_id);
        let test_lotto_id = get_preset_id("TestLotto".to_string()).unwrap();
        assert_eq!(0, test_lotto_id);
    }
}
