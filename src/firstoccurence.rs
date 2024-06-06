use std::io;

pub fn first(){
    let arr = [1, 2, 2, 2, 3, 4, 5];

    print!("Array: [ ");
    for i in arr {
        print!("{}, ", i);
    }
    println!("]");
    
    println!("Enter the target");

    let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let target: i32 = s.trim().parse().unwrap();
    

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_index = mid as usize;
        
        if arr[mid_index] == target {
            result = Some(mid_index);
            high = mid - 1;
        } else if arr[mid_index] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}