#![allow(dead_code)]

pub fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut smallest = i32::MAX;
    for num in arr {
        if *num < smallest {
            smallest = *num;
        }
    }
    smallest
}

fn iter_find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

pub fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 { "even" } else { "odd" }
}

pub fn remove_char(s: &str) -> String {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    String::from(chars.as_str())
}

pub fn opposite(number: i32) -> i32 {
    -number
}

pub fn make_upper_case(s: &str) -> String {
    String::from(s).to_uppercase()
}

pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i * x)
    }
    v
}

pub fn short_count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|e| x * e).collect()
}

pub fn set_alarm(employed: bool, vacation: bool) -> bool {
    employed && !vacation
}

pub fn repeat_string(src: &str, count: usize) -> String {
    src.repeat(count)
}

pub fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.to_digit(10).unwrap() < 5 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}

pub fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|v| -*v).collect()
}

pub fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a + b)
}

pub fn longest(a1: &str, a2: &str) -> String {
    let s = format!("{a1}{a2}");
    let mut v: Vec<char> = s.chars().collect();
    v.sort_unstable();
    v.dedup();
    v.into_iter().collect()
}

pub fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut ret: u32 = 0;
    for (exp, num) in slice.iter().enumerate() {
        ret += num * 2_u32.pow(slice.len() as u32 - exp as u32 - 1);
    }
    ret
}

pub fn binary_slice_to_number_refactor(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |exp, num| (exp << 1) | num)
}

pub fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0_f64;
    }
    let sum = slice.iter().fold(0_f64, |acc, &x| acc + x);
    sum / (slice.len() as f64)
}

pub fn century(year: u32) -> u32 {
    (year - 1) / 100 + 1
}
