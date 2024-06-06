pub fn shortest(){
    let sentence = "God created humanity on the third day";
    println!("Input: {}", sentence);

    match shortest_word(&sentence) {
        Some(word) => println!("The shortest word is '{}'", word),
        None => println!("The input string is empty or has no words"),
    }
}

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
}