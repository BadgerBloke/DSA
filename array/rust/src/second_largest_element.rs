pub fn get_second_largest(arr: &[i32]) -> i32 {
    let mut largest = -1;
    let mut second_largest = -1;

    for e in arr.iter() {
        if *e > largest {
            second_largest = largest;
            largest = *e;
        } else if *e > second_largest {
            second_largest = *e;
        }
    }

    println!("Seccond largest element: {}", second_largest);
    second_largest
}
