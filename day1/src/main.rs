fn main() {
    let text = String::from("asdd5dadd5dfff5eeee9ddff");

    let vector_text = vec!["asdfd5dfaewer4erwew8888855555559", "awrwerew8rewer4werwrewe5ee"];

    let mut final_result: i32 = 0;

    for string in vector_text.iter() {
        final_result += find_and_add_first_last_num(&string);
    }

    println!("Final result = {}", final_result);

}

fn find_and_add_first_last_num(text: &str) -> i32 {
    
    let mut start_num: i32 = 0;
    let mut last_num: i32 = 0;
    let mut result: i32 = 0;
    
    for (_i, c) in text.chars().enumerate() {
        if c.is_digit(10) {
            start_num = c.to_digit(10).unwrap_or(0) as i32;
            break;
        }
    }
    
    for (_i, c) in text.char_indices().rev() {
        if c.is_digit(10) {
            last_num = c.to_digit(10).unwrap_or(0) as i32;
            break;
        }
    }

    println!("Start num: {} Last Num: {}", &start_num, &last_num);

    result = start_num + last_num;

    result

}
