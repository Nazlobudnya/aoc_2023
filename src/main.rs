use itertools::Itertools;
use std::collections::HashMap;

use std::{env, fs};

mod solutions;

use solutions::{day_1, day_10, day_11, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9};

static mut COUNT: usize = 0;

fn get_input_file_name(day: usize, is_test: bool) -> String {
    format!("./src/inputs/{day}.{}", if is_test { "test" } else { "in" })
}

fn day_1(input: String) {
    let (ans_one, ans_two) = day_1::solution(input);

    println!("[Part_1] Sub of calibration values: [{ans_one}] \n[Part_2] Sub of calibration values: [{ans_two}] \n");
}

fn day_2(input: String) {
    let (ans_one, ans_two) = day_2::solution(input);

    println!("[Part_1] Sum of possible games idx: [{ans_one}] \n[Part_2] Sum of possible games: [{ans_two}] \n");
}

fn day_3(input: String) {
    let (ans_one, ans_two) = day_3::solution(input);

    println!("[Part_1] Sum of parts: [{ans_one}] \n[Part_2] Sum of parts: [{ans_two}] \n");
}

fn day_4(input: String) {
    let (ans_one, ans_two) = day_4::solution(input);

    println!(
        "[Part_1] Total winning points: [{ans_one}] \n[Part_2] Winning points: [{ans_two}] \n"
    );
}

fn day_5(input: String) {
    let (ans_one, ans_two) = day_5::solution(input);

    println!("[Part_1] Seed locations: [{ans_one}] \n[Part_2] Seed locations: [{ans_two}] \n");
}

fn day_6(input: String) {
    let (ans_one, ans_two) = day_6::solution(input);

    println!("[Part_1] Product of num ways to beat: [{ans_one}] \n[Part_2] Product of num ways to beat: [{ans_two}] \n");
}

fn day_7(input: String) {
    let (ans_one, ans_two) = day_7::solution(input);

    println!("[Part_1] Total winnings: [{ans_one}] \n[Part_2] Total winnings: [{ans_two}] \n");
}

fn day_8(input: String) {
    let (ans_one, ans_two) = day_8::solution(input);

    println!("[Part_1] Steps: [{ans_one}] \n[Part_2] Steps: [{ans_two}] \n");
}

fn day_9(input: String) {
    let (ans_one, ans_two) = day_9::solution(input);

    println!("[Part_1] Next value in sequence: [{ans_one}] \n[Part_2] Next value: [{ans_two}] \n");
}

fn day_10(input: String) {
    let (ans_one, ans_two) = day_10::solution(input);

    println!("[Part_1] Steps to farthest point of the loop: [{ans_one}] \n[Part_2] Steps to farthest point of the loop: [{ans_two}] \n");
}

fn day_11(input: String) {
    let (ans_one, ans_two) = day_11::solution(input);

    println!("[Part_1] Somestuff: [{ans_one}] \n[Part_2] Some stuff: [{ans_two}] \n");
}

fn main() {
    let mut hm: HashMap<usize, fn(input: String) -> ()> = HashMap::new();
    hm.insert(1, day_1);
    hm.insert(2, day_2);
    hm.insert(3, day_3);
    hm.insert(4, day_4);
    hm.insert(5, day_5);
    hm.insert(6, day_6);
    hm.insert(7, day_7);
    hm.insert(8, day_8);
    hm.insert(9, day_9);
    hm.insert(10, day_10);
    hm.insert(11, day_11);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough args");
    }

    let what = &args.get(1);

    if what.is_none() {
        eprintln!("Day is not supplied. Possible values: [all, {{day_number}}]");
    }

    let is_test = args.get(2);

    let is_test = if is_test.is_none() {
        false
    } else {
        matches!(is_test.unwrap().as_str(), "t" | "test")
    };

    let what = what.unwrap();

    if what == "all" {
        for (&day_num, &func) in hm.iter().sorted() {
            println!(
                "===>DAY {day_num} [{}] \n",
                if is_test { "TEST" } else { "PERSONAL" }
            );

            let contents = fs::read_to_string(get_input_file_name(day_num, is_test))
                .expect("File is not there or unable to read");

            func(contents);
        }
    } else {
        let day_num = what.parse::<usize>().unwrap();
        if let Some(func) = hm.get(&day_num) {
            println!(
                "===>DAY {day_num} [{}] \n",
                if is_test { "TEST" } else { "PERSONAL" }
            );

            let contents = fs::read_to_string(get_input_file_name(day_num, is_test))
                .expect("File is not there or unable to read");

            func(contents);
        } else {
            unimplemented!("Nothing for day ${what}");
        }
    }
}

#[allow(dead_code)]
const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[allow(dead_code)]
fn digits(i: &str) -> [u32; 2] {
    unsafe {
        COUNT += 1;
    }
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = i.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
        i += 1;
    }

    [first.unwrap(), last]
}

#[allow(dead_code)]
fn tr(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits = digits(line);
        sum += digits[0] * 10 + digits[1];
    }

    sum
}
