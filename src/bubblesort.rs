use std::cmp::Ordering;
use super::List;

pub fn bubblesort<T>(list: &mut List<T>)
where T: Ord
{
    let mut swapped = true;
    let mut iterations = 0;
    while swapped {
        swapped = false;
        for index in 1..list.len()-iterations {
            if list.compare(index-1, index) == Ordering::Greater {
                list.swap(index-1, index);
                swapped = true;
            }
        }
        iterations += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::BasicList;
    #[test]
    fn test() {
        let mut test_slice = [1,4,123,7,8,4,2,4,57,8,324,213];
        let mut test_slice2 = test_slice.clone();
        let mut list = BasicList::new(&mut test_slice2);
        assert_eq!(test_slice, list.slice);
        test_slice.sort_unstable();
        bubblesort(&mut list);
        assert_eq!(test_slice, list.slice);
        println!("{:?}", test_slice2);
    }
}
