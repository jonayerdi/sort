pub mod bubblesort;
pub mod heapsort;
pub mod insertionsort;
pub mod quicksort;
pub mod quicksort2;
pub mod selectionsort;
pub mod shellsort;

pub use bubblesort::*;
pub use insertionsort::*;
pub use quicksort::*;
pub use quicksort2::*;
pub use selectionsort::*;
pub use shellsort::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    Get(usize),
    Set(usize),
    Compare(usize, usize),
    Swap(usize, usize),
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operation::Get(index) => write!(f, "Get[{}]", index),
            Operation::Set(index) => write!(f, "Set[{}]", index),
            Operation::Compare(index1, index2) => write!(f, "Compare[{}][{}]", index1, index2),
            Operation::Swap(index1, index2) => write!(f, "Swap[{}][{}]", index1, index2),
        }
    }
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub trait List<T>
where
    T: Copy + Ord,
{
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn get(&self, index: usize) -> T {
        self.as_slice()[index]
    }
    fn set(&mut self, index: usize, value: T) {
        self.as_mut_slice()[index] = value;
    }
    fn compare(&self, a: usize, b: usize) -> std::cmp::Ordering {
        self.get(a).cmp(&self.get(b))
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
}

impl<T> List<T> for Vec<T>
where
    T: Copy + Ord,
{
    fn as_slice(&self) -> &[T] {
        self
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

pub struct CallbackList<'a, 'b, T>
where
    T: Copy + Ord,
{
    pub slice: &'a mut [T],
    pub callback: Box<dyn 'b + Fn(Operation, &[T])>,
}
impl<'a, 'b, T> CallbackList<'a, 'b, T>
where
    T: Copy + Ord,
{
    pub fn new(
        slice: &'a mut [T],
        callback: Box<dyn 'b + Fn(Operation, &[T])>,
    ) -> CallbackList<'a, 'b, T> {
        CallbackList { slice, callback }
    }
}
impl<'a, 'b, T> List<T> for CallbackList<'a, 'b, T>
where
    T: Copy + Ord,
{
    fn as_slice(&self) -> &[T] {
        self.slice
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.slice
    }
    fn get(&self, index: usize) -> T {
        let result = self.slice[index];
        (self.callback)(Operation::Get(index), self.slice);
        result
    }
    fn set(&mut self, index: usize, value: T) {
        self.slice[index] = value;
        (self.callback)(Operation::Set(index), self.slice);
    }
    fn compare(&self, a: usize, b: usize) -> std::cmp::Ordering {
        let result = self.slice[a].cmp(&self.slice[b]);
        (self.callback)(Operation::Compare(a, b), self.slice);
        result
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.slice.swap(a, b);
        (self.callback)(Operation::Swap(a, b), self.slice);
    }
}
