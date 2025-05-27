use max_product_of_a_triplet::get_max_product_of_a_triplet;
use move_all_zeros_to_the_end::move_all_zero_to_end;
use second_largest_element::get_second_largest;
use third_largest_element::get_third_largest;
mod max_product_of_a_triplet;
mod move_all_zeros_to_the_end;
mod second_largest_element;
mod third_largest_element;

fn main() {
    // let arr = [2, 4, 5, 1, 9, 6];
    // let arr = [10, 3, 5, 6, 20];
    let arr = [0, -4, 0, -6, 7, 0];
    // let arr = [-10, -3, -5, -6, -20];

    get_second_largest(&arr);
    get_third_largest(&arr);
    get_max_product_of_a_triplet(&arr);
    move_all_zero_to_end(&arr);
}
