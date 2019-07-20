use std::cmp::Ordering;
use super::List;

pub fn selectionsort<T>(list: &mut List<T>)
where T: Ord
{
    for index1 in 0..list.len() {
        let mut min_index = index1;
        for index2 in index1+1..list.len() {
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
    use super::super::BasicList;
    #[test]
    fn test_selectionsort() {
        let mut test_slice = [1,4,123,7,8,4,2,4,57,8,324,213];
        let mut test_slice2 = test_slice.clone();
        let mut list = BasicList::new(&mut test_slice2);
        assert_eq!(test_slice, list.slice);
        test_slice.sort_unstable();
        selectionsort(&mut list);
        assert_eq!(test_slice, list.slice);
        println!("{:?}", list.slice);
    }
}
