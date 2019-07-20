use std::cmp::Ordering;
use super::List;

use rand::prelude::*;

fn relocate_pivot_right<T>(list: &mut List<T>, pivot: usize, right: usize) -> usize
where T: Ord
{
    let mut pivot = pivot;
    loop {
        let mut swapped = false;
        for i in 1..=right - pivot {
            if list.compare(pivot, pivot + i) == Ordering::Greater {
                list.swap(pivot + 1, pivot + i);
                list.swap(pivot, pivot + 1);
                pivot += 1;
                swapped = true;
                break;
            }
        };
        if !swapped {
            break;
        }
    }
    pivot
}

fn relocate_pivot_left<T>(list: &mut List<T>, left: usize, pivot: usize) -> usize
where T: Ord
{
    let mut pivot = pivot;
    loop {
        let mut swapped = false;
        for i in (1..=pivot - left).rev() {
            if list.compare(pivot - i, pivot) == Ordering::Greater {
                list.swap(pivot - i, pivot - 1);
                list.swap(pivot - 1, pivot);
                pivot -= 1;
                swapped = true;
                break;
            }
        };
        if !swapped {
            break;
        }
    }
    pivot
}

fn swap_from_sides<T>(list: &mut List<T>, begin: usize, pivot: usize, end: usize) -> usize
where T: Ord
{
    let mut left = begin;
    let mut right = end;
    loop {
        while list.compare(left, pivot) != Ordering::Greater {
            if left == pivot {
                return right;
            }
            left += 1;
        }
        while list.compare(pivot, right) != Ordering::Greater {
            if right == pivot {
                return left;
            }
            right -= 1;
        }
        list.swap(left, right);
    };
}

fn partition<T>(list: &mut List<T>, begin: usize, end: usize) -> usize
where T: Ord
{
    let pivot: usize = random::<usize>() % (end+1-begin);
    // Swap large elements to the left with small ones to the right
    let remaining = swap_from_sides(list, begin, pivot, end);
    // Move remaining elements left or right and reposition pivot element
    if remaining < pivot {
        relocate_pivot_left(list, remaining, pivot)
    } else {
        relocate_pivot_right(list, pivot, remaining)
    }
}

fn real_quicksort<T>(list: &mut List<T>, begin: usize, end: usize)
where T: Ord
{
    if begin < end {
        // Partition elements
        let pivot = partition(list, begin, end);
        // Recursion (tail call to the largest partition)
        if pivot-begin <= end-pivot {
            real_quicksort(list, begin, pivot-1);
            real_quicksort(list, pivot+1, end);
        } else {
            real_quicksort(list, pivot+1, end);
            real_quicksort(list, begin, pivot-1);
        }
    }
}

pub fn quicksort<T>(list: &mut List<T>)
where T: Ord
{
    let length = list.len();
    real_quicksort(list, 0, length-1);
}
