use super::List;
use std::cmp::Ordering;

pub fn insertionsort<T>(list: &mut dyn List<T>)
where
    T: Copy + Ord + std::fmt::Display,
{
    for insert_index in 1..list.len() {
        let mut swap_index = insert_index;
        while swap_index > 0 && list.compare(swap_index - 1, swap_index) == Ordering::Greater {
            list.swap(swap_index - 1, swap_index);
            swap_index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertionsort() {
        let mut test_slice = vec![1, 4, 123, 7, 8, 4, 8, 8, 2, 4, 57, 8, 324, 213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        insertionsort(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
