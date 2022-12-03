use std::fs;

pub fn get_input(i: i32) -> String {
    fs::read_to_string(format!("./input/{}", i)).expect(format!("No input file {}", i).as_str())
}
