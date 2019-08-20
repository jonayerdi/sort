use super::List;
use std::cmp::Ordering;

use rand::prelude::*;

fn relocate_pivot_right<T>(list: &mut dyn List<T>, pivot: usize, right: usize) -> usize
where
    T: Copy + Ord + std::fmt::Display,
{
    let mut count = 0;
    if pivot < right {
        for i in pivot + 1..=right {
            if list.compare(pivot, i) == Ordering::Greater {
                count += 1;
            }
        }
    }
    pivot + count
}

fn relocate_pivot_left<T>(list: &mut dyn List<T>, left: usize, pivot: usize) -> usize
where
    T: Copy + Ord + std::fmt::Display,
{
    let mut count = 0;
    if left < pivot {
        for i in left..=pivot - 1 {
            if list.compare(i, pivot) == Ordering::Greater {
                count += 1;
            }
        }
    }
    pivot - count
}

fn swap_from_sides<T>(list: &mut dyn List<T>, begin: usize, pivot: usize, end: usize) -> usize
where
    T: Copy + Ord + std::fmt::Display,
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
    }
}

fn partition<T>(list: &mut dyn List<T>, begin: usize, end: usize) -> usize
where
    T: Copy + Ord + std::fmt::Display,
{
    let pivot = (random::<usize>() % (end + 1 - begin)) + begin;
    // Swap large elements to the left with small ones to the right
    let remaining = swap_from_sides(list, begin, pivot, end);
    // Calculate final position of pivot element
    let (left, new_pivot, right) = if remaining < pivot {
        let new_pivot = relocate_pivot_left(list, remaining, pivot);
        (remaining, new_pivot, pivot)
    } else {
        let new_pivot = relocate_pivot_right(list, pivot, remaining);
        (pivot, new_pivot, remaining)
    };
    // Reposition pivot
    list.swap(pivot, new_pivot);
    // Swap remaining element to the correct side
    swap_from_sides(list, left, new_pivot, right);
    // Return final pivot position
    new_pivot
}

fn real_quicksort<T>(list: &mut dyn List<T>, begin: usize, end: usize)
where
    T: Copy + Ord + std::fmt::Display,
{
    if begin < end {
        // Partition elements
        let pivot = partition(list, begin, end);
        // Calculate element count on each side of the pivot
        let left_length = pivot - begin;
        let right_length = end - pivot;
        // Recursion (tail call to the largest partition)
        if left_length <= right_length {
            if left_length > 0 {
                real_quicksort(list, begin, pivot - 1);
            }
            real_quicksort(list, pivot + 1, end);
        } else {
            if right_length > 0 {
                real_quicksort(list, pivot + 1, end);
            }
            real_quicksort(list, begin, pivot - 1);
        }
    }
}

pub fn quicksort2<T>(list: &mut dyn List<T>)
where
    T: Copy + Ord + std::fmt::Display,
{
    let length = list.len();
    if length > 1 {
        real_quicksort(list, 0, length - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quicksort2() {
        let mut test_slice = vec![1, 4, 123, 7, 8, 4, 2, 4, 57, 8, 324, 213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        quicksort2(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
