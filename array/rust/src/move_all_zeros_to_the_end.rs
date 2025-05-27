pub fn move_all_zero_to_end(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    let mut count = 0;

    for i in 0..arr.len() {
        if arr[i] != 0 {
            arr.swap(count, i);
            count += 1;
        }
    }

    println!("All zeros moved to end: {:?}", arr);
    arr
}
