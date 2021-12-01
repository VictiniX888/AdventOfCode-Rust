use std::fs;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("input/day{:02}.txt", day)).expect("Failed to read input file!")
}
