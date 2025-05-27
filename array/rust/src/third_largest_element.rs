pub fn get_third_largest(arr: &[i32]) -> i32 {
    let mut largest = -1;
    let mut second_largest = -1;
    let mut third_largest = -1;

    for e in arr {
        if e > &largest {
            third_largest = second_largest;
            second_largest = largest;
            largest = *e;
        } else if e > &second_largest {
            third_largest = second_largest;
            second_largest = *e;
        } else if e > &third_largest {
            third_largest = *e;
        }
    }

    println!("Third largest element: {}", third_largest);

    third_largest
}
