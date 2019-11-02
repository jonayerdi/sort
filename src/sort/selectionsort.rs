use super::List;
use std::cmp::Ordering;

pub fn selectionsort<T>(list: &mut dyn List<T>)
where
    T: Copy + Ord + std::fmt::Display,
{
    for index1 in 0..list.len() {
        let mut min_index = index1;
        for index2 in index1 + 1..list.len() {
            if list.compare(min_index, index2) == Ordering::Greater {
                min_index = index2;
            }
        }
        if min_index != index1 {
            list.swap(index1, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selectionsort() {
        let mut test_slice = vec![1, 4, 123, 7, 8, 4, 8, 8, 2, 4, 57, 8, 324, 213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        selectionsort(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
