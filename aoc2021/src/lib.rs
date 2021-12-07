use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("input/day{:02}.txt", day)).expect("Failed to read input file!")
}

pub fn parse_str<T>(str: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    str.parse().unwrap()
}
