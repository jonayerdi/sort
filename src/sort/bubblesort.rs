use super::List;
use std::cmp::Ordering;

pub fn bubblesort<T>(list: &mut dyn List<T>)
where
    T: Copy + Ord + std::fmt::Display,
{
    let mut swapped = true;
    let mut iterations = 0;
    while swapped {
        swapped = false;
        for index in 1..list.len() - iterations {
            if list.compare(index - 1, index) == Ordering::Greater {
                list.swap(index - 1, index);
                swapped = true;
            }
        }
        iterations += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubblesort() {
        let mut test_slice = vec![1, 4, 123, 7, 8, 4, 2, 4, 57, 8, 324, 213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        bubblesort(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
