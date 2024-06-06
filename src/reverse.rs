pub fn reverse(){
    let original_string = "retrograde";
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}