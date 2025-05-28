pub fn get_max_consecutive_ones_count(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    };

    let mut max_count = 1;
    let mut count = 1;

    for window in arr.windows(2) {
        if window[0] == window[1] {
            count += 1;
            max_count = max_count.max(count);
        } else {
            count = 1;
        }
    }

    println!("Max consecutive ones count: {}", max_count);
    Some(max_count)
}
