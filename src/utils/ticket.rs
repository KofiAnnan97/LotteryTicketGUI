use std::vec;
use rand::rng;


pub fn generate(limits: &Vec<i32>) -> Vec<i32> {
    let mut numbers : Vec<i32> = Vec::new();
    //let mut rng = rand::rng();
    for i in 0..limits.len(){
        numbers.push(rand::random_range(1..limits[i]));
    }
    return numbers;
}