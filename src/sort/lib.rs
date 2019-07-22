pub mod bubblesort;
pub mod selectionsort;
pub mod quicksort;
pub mod quicksort2;
pub mod shellsort;
pub mod heapsort;

#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    Compare,
    Swap,
}
impl std::fmt::Display for Operation {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Operation::Compare => "Compare",
            Operation::Swap => "Swap",
        })
    }
}
impl std::fmt::Debug for Operation {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub struct Step {
    pub operation: Operation,
    pub indices: [usize; 2],
}
impl std::fmt::Display for Step {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}({},{})", self.operation, self.indices[0], self.indices[1])
    }
}
impl std::fmt::Debug for Step {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub trait List<T> {
    fn len(&self) -> usize;
    fn compare(&mut self, a: usize, b: usize)  -> std::cmp::Ordering where T: Ord;
    fn swap(&mut self, a: usize, b: usize);
}

pub struct BasicList<'a,T> {
    pub slice: &'a mut [T],
}
impl<'a,T> BasicList<'a,T> {
    pub fn new(slice: &mut [T]) -> BasicList<T> {
        BasicList { slice }
    }
}
impl<'a,T> List<T> for BasicList<'a,T> {
    fn len(&self) -> usize {
        self.slice.len()
    }
    fn compare(&mut self, a: usize, b: usize)  -> std::cmp::Ordering
    where T: Ord
    {
        self.slice[a].cmp(&self.slice[b])
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.slice.swap(a, b);
    }
}

pub struct RecorderList<'a,T> {
    pub slice: &'a mut [T],
    pub steps: Vec<Step>,
}
impl<'a,T> RecorderList<'a,T> {
    pub fn new(slice: &mut [T]) -> RecorderList<T> {
        RecorderList { slice, steps: Vec::new() }
    }
}
impl<'a,T> List<T> for RecorderList<'a,T> {
    fn len(&self) -> usize {
        self.slice.len()
    }
    fn compare(&mut self, a: usize, b: usize)  -> std::cmp::Ordering
    where T: Ord
    {
        self.steps.push(Step { operation: Operation::Compare, indices: [a,b] });
        self.slice[a].cmp(&self.slice[b])
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.steps.push(Step { operation: Operation::Swap, indices: [a,b] });
        self.slice.swap(a, b);
    }
}
