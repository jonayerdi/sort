use std::cmp::Ordering;
use super::List;

use rand::prelude::*;

fn relocate_pivot_right<T>(list: &mut List<T>, pivot: usize, right: usize) -> usize
where T: Copy + Ord + std::fmt::Display
{
    let mut pivot = pivot;
    while pivot < right {
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
where T: Copy + Ord + std::fmt::Display
{
    let mut pivot = pivot;
    while left < pivot {
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
where T: Copy + Ord + std::fmt::Display
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
where T: Copy + Ord + std::fmt::Display
{
    let pivot = (random::<usize>() % (end+1-begin)) + begin;
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
where T: Copy + Ord + std::fmt::Display
{
    if begin < end {
        // Partition elements
        let pivot = partition(list, begin, end);
        // Calculate element count on each side of the pivot
        let left_length = pivot-begin;
        let right_length = end-pivot;
        // Recursion (tail call to the largest partition)
        if left_length <= right_length {
            if left_length > 0 {
                real_quicksort(list, begin, pivot-1);
            }
            real_quicksort(list, pivot+1, end);
        } else {
            if right_length > 0 {
                real_quicksort(list, pivot+1, end);
            }
            real_quicksort(list, begin, pivot-1);
        }
    }
}

pub fn quicksort<T>(list: &mut List<T>)
where T: Copy + Ord + std::fmt::Display
{
    let length = list.len();
    real_quicksort(list, 0, length-1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quicksort() {
        let mut test_slice = vec![1,4,123,7,8,4,2,4,57,8,324,213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        quicksort(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
