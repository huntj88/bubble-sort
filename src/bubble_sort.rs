use std::clone::Clone;
use std::cmp::{Ord, Ordering};

use crate::bubble_sort::SortStatus::{Sorted, Unsorted};

pub(crate) fn sort<T: Ord + Clone>(list: &Vec<T>) -> Vec<T> {
    let mut vec = list.to_vec();

    loop {
        match reorder_one_pass(vec) {
            Sorted(after) => {
                return after;
            }
            Unsorted(after) => {
                vec = after;
            }
        }
    }
}

fn reorder_one_pass<T: Ord>(mut list: Vec<T>) -> SortStatus<T> {
    let mut index = 0;
    let mut swapped_any = false;
    while let Some(current) = list.get(index) {
        if let Some(next) = list.get(index + 1) {
            if let Ordering::Greater = current.cmp(next) {
                swapped_any = true;
                list.swap(index, index + 1);
            }
        }
        index += 1
    };

    if swapped_any {
        Unsorted(list)
    } else {
        Sorted(list)
    }
}

enum SortStatus<T> {
    Sorted(Vec<T>),
    Unsorted(Vec<T>),
}
