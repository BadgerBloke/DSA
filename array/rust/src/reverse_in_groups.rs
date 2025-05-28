pub fn reverse_in_groups(arr: &[i32], size: usize) -> Vec<i32> {
    let mut arr = arr.to_vec();
    let n = arr.len();

    for i in (0..n).step_by(size) {
        let mut left = i;
        let mut right = (i + size - 1).min(n - 1);

        while left < right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    println!("Reversed array: {:?}", arr);
    arr
}
