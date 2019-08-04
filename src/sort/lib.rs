pub mod bubblesort;
pub mod selectionsort;
pub mod quicksort;
pub mod quicksort2;
pub mod shellsort;
pub mod heapsort;

#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    Get(usize),
    Set(usize),
    Compare(usize,usize),
    Swap(usize,usize),
}
impl std::fmt::Display for Operation {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operation::Get(index) => write!(f, "Get[{}]", index),
            Operation::Set(index) => write!(f, "Set[{}]", index),
            Operation::Compare(index1, index2) => write!(f, "Compare[{}][{}]", index1, index2),
            Operation::Swap(index1, index2) => write!(f, "Swap[{}][{}]", index1, index2),
        }
    }
}
impl std::fmt::Debug for Operation {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

pub trait List<T> 
where T: Copy + Ord 
{
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> T;
    fn set(&mut self, index: usize, value: T);
    fn compare(&self, a: usize, b: usize)  -> std::cmp::Ordering;
    fn swap(&mut self, a: usize, b: usize);
}

impl<T> List<T> for Vec<T>
where T: Copy + Ord
{
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> T {
        self[index]
    }
    fn set(&mut self, index: usize, value: T) {
        self[index] = value;
    }
    fn compare(&self, a: usize, b: usize) -> std::cmp::Ordering {
        self[a].cmp(&self[b])
    }
    fn swap(&mut self, a: usize, b: usize) {
        // Copypasted from Vec::swap
        unsafe {
            // Can't take two mutable loans from one vector, so instead just cast
            // them to their raw pointers to do the swap
            let pa: *mut T = &mut self[a];
            let pb: *mut T = &mut self[b];
            std::ptr::swap(pa, pb);
        }
    }
}

pub struct CallbackList<'a,'b,T>
where T: Copy + Ord
{
    pub slice: &'a mut [T],
    pub callback: Box<Fn(Operation,&[T]) + 'b>,
}
impl<'a,'b,T> CallbackList<'a,'b,T>
where T: Copy + Ord
{
    pub fn new(slice: &'a mut [T], callback: Box<Fn(Operation,&[T]) + 'b>) -> CallbackList<'a,'b,T> {
        CallbackList { slice, callback }
    }
}
impl<'a,'b,T> List<T> for CallbackList<'a,'b,T>
where T: Copy + Ord
{
    fn len(&self) -> usize {
        self.slice.len()
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
    fn compare(&self, a: usize, b: usize) -> std::cmp::Ordering
    {
        let result = self.slice[a].cmp(&self.slice[b]);
        (self.callback)(Operation::Compare(a,b), self.slice);
        result
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.slice.swap(a, b);
        (self.callback)(Operation::Swap(a,b), self.slice);
    }
}
