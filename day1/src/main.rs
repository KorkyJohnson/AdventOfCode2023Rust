fn main() {
    let text = String::from("asdd5dadd5dfff5eeee9ddff");

    let mut start_num:i32 = 0;
    let mut last_num: i32 = 0;

    for (i, c) in text.chars().enumerate(){
        if c.is_digit(10){
            start_num = c.to_digit(10).unwrap_or(0) as i32;
            break;
        }
    }
    
    for (i, c) in text.char_indices().rev(){
        if c.is_digit(10){
            last_num= c.to_digit(10).unwrap_or(0) as i32;
            break;
        }
    }

    println!("Start num: {} Last Num: {}", start_num, last_num)
}
