use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

pub fn get_date_str() -> String {
    let now = SystemTime::now();
    let dt: DateTime<Utc> = now.into();
    dt.format("%d/%m/%Y").to_string()
}

pub fn get_time_str() -> String {
    let now = SystemTime::now();
    let dt: DateTime<Utc> = now.into();
    dt.format("%T").to_string()
}

pub fn convert_vec_to_str(nums: Vec<i32>) -> String {
    let mut temp : String = String::from("");
    for n in 0..nums.len()-1{
        temp += &format!("{},", nums[n].to_string());
    }
    temp += &nums[nums.len()-1].to_string();
    temp
}

pub fn convert_str_to_vec(nums_str: &String) -> Vec<i32> {
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
