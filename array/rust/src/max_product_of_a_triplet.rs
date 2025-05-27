pub fn get_max_product_of_a_triplet(arr: &[i32]) -> i32 {
    let mut max_a = i32::MIN;
    let mut max_b = i32::MIN;
    let mut max_c = i32::MIN;

    let mut min_a = i32::MAX;
    let mut min_b = i32::MAX;

    for e in arr {
        if e > &max_a {
            max_c = max_b;
            max_b = max_a;
            max_a = *e;
        } else if e > &max_b {
            max_c = max_b;
            max_b = *e;
        } else if e > &max_c {
            max_c = *e;
        }

        if e < &min_a {
            min_b = min_a;
            min_a = *e;
        } else if e < &min_b {
            min_b = *e;
        }
    }

    let prod_a = max_a * max_b * max_c;
    let prod_b = min_a * min_b * max_a;

    let product = if prod_a > prod_b { prod_a } else { prod_b };

    println!("Maximum product of a tiplet: {}", product);
    product
}
