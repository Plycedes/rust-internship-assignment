pub fn median(){
    let arr = [1, 2, 3, 4, 5, 6];    

    match find_median(&arr) {
        Some(median) => println!("Array: {:?}\nMedian: {}", arr, median),
        None => println!("The array {:?} is empty", arr),
    }
}

fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    
    if len % 2 == 0 {
        let mid1 = len / 2 - 1;
        let mid2 = len / 2;
        Some((arr[mid1] as f64 + arr[mid2] as f64) / 2.0)
    } else {
        let mid = len / 2;
        Some(arr[mid] as f64)
    }
}