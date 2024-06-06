use std::io;

pub fn palindrome(){
    println!("Enter a string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    if is_palindrome(&input) {
        println!("'{}' is a palindrome", input);
    } else {
        println!("'{}' is not a palindrome", input);
    } 
}

fn is_palindrome(s: &str) -> bool{
    let my_chars: Vec<char> = s.chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_lowercase().next().unwrap())
    .collect();
    let len = my_chars.len();
    for i in 0..len / 2 {
        if my_chars[i] != my_chars[len - 1 - i] {            
            return false;
        }
    }
    true
}