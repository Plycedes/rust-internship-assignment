pub fn maxsum(){
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

    print!("Array: [ ");
    for i in arr {
        print!("{}, ", i);
    }
    println!("]");

    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;

    for &num in arr {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}