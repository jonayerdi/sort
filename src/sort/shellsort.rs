use super::List;
use std::cmp::Ordering;

struct TokudaIterator {
    max_value: usize,
    pow_value: f64,
}

impl TokudaIterator {
    fn new(max_value: usize) -> TokudaIterator {
        TokudaIterator {
            max_value,
            pow_value: 1.0,
        }
    }
}

impl Iterator for TokudaIterator {
    type Item = usize;
    fn next (&mut self) -> Option<Self::Item> {
        self.pow_value *= 2.25;
        let value = (0.8f64 * (self.pow_value as f64 - 1.0)).ceil() as usize;
        if value <= self.max_value {
            Some(value)
        } else {
            None
        }
    }
}

pub fn gaps_sequence(len: usize) -> impl Iterator<Item=usize> {
    TokudaIterator::new(len).collect::<Vec<_>>().into_iter().rev()
}

pub fn shellsort<T>(list: &mut dyn List<T>)
where
    T: Copy + Ord + std::fmt::Display,
{
    for gap in gaps_sequence(list.len() - 1) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokuda_sequence() {
        let expected = vec![1, 4, 9, 20, 46, 103, 233, 525, 1182, 2660, 5985, 13467];
        let actual = TokudaIterator::new(*expected.last().unwrap_or(&0)).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_shellsort() {
        let mut test_slice = vec![1, 4, 123, 7, 8, 4, 8, 8, 2, 4, 57, 8, 324, 213];
        let mut test_slice2 = test_slice.clone();
        assert_eq!(test_slice, test_slice2);
        test_slice.sort_unstable();
        shellsort(&mut test_slice2);
        assert_eq!(test_slice, test_slice2);
        println!("{:?}", test_slice2);
    }
}
