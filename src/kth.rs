use std::io;

pub fn kth(){
    let arr = [12, 9, 1, 5, 4];

    print!("Array: [ ");
    for i in arr {
        print!("{}, ", i);
    }
    println!("]");

    println!("Enter the element kth element");

    let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let k: usize = s.trim().parse().unwrap();

    match kth_smallest(&arr, k) {
        Some(value) => println!("The {}-th smallest element is {}", k, value),
        None => println!("Invalid value of k"),
    }
}

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}