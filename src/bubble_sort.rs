use std::clone::Clone;
use std::cmp::{Ord, Ordering};

pub(crate) fn sort<T: Ord>(mut list: &mut [T]) {
    loop {
        if is_sorted_or_sort_one_pass(list) {
            break;
        }
    }
}

fn is_sorted_or_sort_one_pass<T: Ord>(list: &mut [T]) -> bool {
    let mut index = 0;
    let mut swapped_any = false;
    while let Some(current) = list.get(index) {
        if let Some(next) = list.get(index + 1) {
            if current > next {
                swapped_any = true;
                list.swap(index, index + 1);
            }
        }
        index += 1
    };

    !swapped_any
}
