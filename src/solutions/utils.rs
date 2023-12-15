use std::{num::ParseIntError, str::FromStr};

use regex::Regex;

pub fn str_get_numbers_regex<T: FromStr<Err = ParseIntError>>(s: &str) -> Vec<T> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(s)
        .map(|n| n[0].parse::<T>().expect("Has to be a number"))
        .collect()
}

pub fn str_get_numbers_delim<T: FromStr<Err = ParseIntError>>(s: &str, delim: &str) -> Vec<T> {
    s.trim()
        .split(delim)
        .map(|x| x.parse::<T>().expect("Has to be a number"))
        .collect()
}
