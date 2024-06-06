mod kth;
mod prime;
mod depth;
mod prefix;
mod median;
mod merged;
mod maxsum;
mod reverse;
mod palindrome;
mod shortestword;
mod firstoccurence;


use std::io;

fn main() {    
    let f = vec![
                    "kth",                      
                    "prime",
                    "first",
                    "depth",
                    "prefix",
                    "median",
                    "merged",
                    "maxsum",
                    "reverse",
                    "shortest",
                    "palindrome",                    
                ];

    println!("====================================================");
    println!("Enter the name of the operation you want to perform!");
    println!("Enter ls to list all the operations!");
    println!("Enter exit to terminate the program");
    println!("====================================================");
    
    let mut exec = true;

    
    while exec {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        
        if input.trim() == "ls" {
            println!("====================================================");
            println!("OPERATIONS:");
            let mut i = 0;
            while i != f.len() {
                println!("{}",f[i]);
                i += 1;
            }            
        }        
        else {            
            match input.trim() {
                "exit" => exec = false,
                "kth" => kth::kth(),                
                "prime" => prime::prime(),
                "depth" => depth::depth(),
                "median" => median::median(),
                "prefix" => prefix::prefix(),
                "merged" => merged::merged(),
                "maxsum" => maxsum::maxsum(),
                "reverse" => reverse::reverse(),                
                "first" => firstoccurence::first(),                
                "shortest" => shortestword::shortest(),
                "palindrome" => palindrome::palindrome(),
                _ => println!("Enter a valid input"),
            }
        }
        println!("====================================================");
    }    
    println!("Terminated");
}