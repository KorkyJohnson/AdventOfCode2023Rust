use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use aho_corasick::AhoCorasick;

fn main() {
    let text = String::from("asdd5dadd5dfff5eeee9ddff");

    let vector_text = vec![
        "asdfd5dfaewer4erwew8888855555559",
        "awrwerew8rewer4werwrewe5ee",
    ];

    let mut final_result: i32 = 0;

    let file = File::open(Path::new("input.txt")).unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    // println!("{}", lines);

    // for string in vector_text.iter() {
    // for string in lines.iter() {
    //     final_result += find_and_add_first_last_num(&string);
    // }
    let mut final_result = 0;
    for line in lines.iter() {
        let nums = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();

        let first_num = *nums.iter().nth(0).unwrap();
        let last_num = *nums.iter().last().unwrap();
        final_result += (first_num as i32 * 10) + last_num as i32;
    }

    println!("Part 1 Final result = {}", final_result);

    let patterns = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    // let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac = AhoCorasick::new(patterns).unwrap();

    let mut total = 0;

    for line in lines.iter() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first_num_idx = matches.iter().nth(0).unwrap().pattern().as_usize();
        let last_num_idx = matches.iter().last().unwrap().pattern().as_usize();

        let first_num = matches.iter().nth(0).unwrap().pattern().as_usize() / 2 + 1;
        let last_num = matches.iter().last().unwrap().pattern().as_usize() / 2 + 1;

        println!("Line: {line} FirstNumIdx: {first_num_idx} FirstNum: {first_num} LastNumIdx: {last_num_idx} LastNum: {last_num}");

        total += (first_num * 10) + last_num;
    }

    println!("Part 2 Final result = {total}");
}

fn find_and_add_first_last_num(text: &str) -> i32 {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut all_numbers_lines: Vec<u8> = vec![];

    let ac = AhoCorasick::new(patterns).unwrap();
    ac.try_stream_replace_all(text.as_bytes(), &mut all_numbers_lines, replace_with)
        .expect("stream_replace_all failed");

    let num_text = String::from_utf8(all_numbers_lines).expect("Bytes not in utf8");

    println!("Text    : {}", text);
    println!("num_text: {}", num_text);

    let first_num: i32 = 0;
    let last_num: i32 = 0;
    let number: i32 = 0;

    // // for (_i, c) in text.chars().enumerate() {
    // for (_i, c) in num_text.chars().enumerate() {
    //     if c.is_digit(10) {
    //         first_num = c.to_digit(10).unwrap_or(0) as i32;
    //         break;
    //     }
    // }

    // // for (_i, c) in text.char_indices().rev() {
    // for (_i, c) in num_text.char_indices().rev() {
    //     if c.is_digit(10) {
    //         last_num = c.to_digit(10).unwrap_or(0) as i32;
    //         break;
    //     }
    // }

    // number = (first_num * 10) + last_num;

    println!(
        "Start num: {} Last Num: {} Hidden Number: {}",
        &first_num, &last_num, &number
    );

    number
}
