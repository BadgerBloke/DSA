mod max_consecutive_ones;
mod max_product_of_a_triplet;
mod move_all_zeros_to_the_end;
mod reverse_in_groups;
mod second_largest_element;
mod third_largest_element;

use max_consecutive_ones::get_max_consecutive_ones_count;
use max_product_of_a_triplet::get_max_product_of_a_triplet;
use move_all_zeros_to_the_end::move_all_zero_to_end;
use reverse_in_groups::reverse_in_groups;
use second_largest_element::get_second_largest;
use third_largest_element::get_third_largest;

fn main() {
    // let arr = [2, 4, 5, 1, 9, 6];
    // let arr = [10, 3, 5, 6, 20];
    let arr = [0, -4, 0, -6, 7, 0];
    // let arr = [-10, -3, -5, -6, -20];

    get_second_largest(&arr);
    get_third_largest(&arr);
    get_max_product_of_a_triplet(&arr);
    move_all_zero_to_end(&arr);

    let arr = [0, 0, 0, 0];
    get_max_consecutive_ones_count(&arr);

    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let size = 10;
    reverse_in_groups(&arr, size);
}
