use std::collections::BinaryHeap;

pub struct PriorityQueue<T, U> where T: Ord + PartialOrd + Eq{
    heap : BinaryHeap<PriorityQueueItem<T,U>>
}

impl<T,U> PriorityQueue<T, U> where T: Ord + PartialOrd + Eq{
    pub fn new() -> Self{
        PriorityQueue{
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, priority: T, item: U) {
        self.heap.push(PriorityQueueItem::new(priority,item));
    }

    pub fn pop_priority(&mut self) -> Option<T> {
        self.heap.pop().and_then(|x|{Some(x.priority)})
    }

    pub fn pop_item(&mut self) -> Option<U> {
        self.heap.pop().and_then(|x|{Some(x.item)})
    }

    pub fn pop(&mut self) -> Option<(T,U)> {
        self.heap.pop().and_then(|x|{Some((x.priority,x.item))})
    }

    pub fn peek_priority(&mut self) -> Option<&T> {
        self.heap.peek().and_then(|x|{Some(&x.priority)})
    }

    pub fn peek_item(&mut self) -> Option<&U> {
        self.heap.peek().and_then(|x|{Some(&x.item)})
    }

    pub fn peek(&mut self) -> Option<(&T,&U)> {
        self.heap.peek().and_then(|x|{Some((&x.priority,&x.item))})
    }
}

pub struct ReversePriorityQueue<T, U> where T: Ord + PartialOrd + Eq{
    heap : BinaryHeap<ReversePriorityQueueItem<T,U>>
}

impl<T,U> ReversePriorityQueue<T, U> where T: Ord + PartialOrd + Eq{
    pub fn new() -> Self{
        ReversePriorityQueue{
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, priority: T, item: U) {
        self.heap.push(ReversePriorityQueueItem::new(priority,item));
    }

    pub fn pop_priority(&mut self) -> Option<T> {
        self.heap.pop().and_then(|x|{Some(x.priority)})
    }

    pub fn pop_item(&mut self) -> Option<U> {
        self.heap.pop().and_then(|x|{Some(x.item)})
    }

    pub fn pop(&mut self) -> Option<(T,U)> {
        self.heap.pop().and_then(|x|{Some((x.priority,x.item))})
    }

    pub fn peek_priority(&mut self) -> Option<&T> {
        self.heap.peek().and_then(|x|{Some(&x.priority)})
    }

    pub fn peek_item(&mut self) -> Option<&U> {
        self.heap.peek().and_then(|x|{Some(&x.item)})
    }

    pub fn peek(&mut self) -> Option<(&T,&U)> {
        self.heap.peek().and_then(|x|{Some((&x.priority,&x.item))})
    }
}


use std::cmp::Ordering;
use std::fmt;

struct PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    priority: T,
    item: U
}

impl<T, U> PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    fn new(priority: T, item: U) -> Self where T: Ord + PartialOrd + Eq {
        PriorityQueueItem {
            priority,
            item
        }
    }
}

impl<T, U> Ord for PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T, U> PartialOrd for PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T, U> PartialEq for PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T, U> Eq for PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {}

impl<T: fmt::Debug, U> fmt::Debug for PriorityQueueItem<T, U> where T: Ord + PartialOrd + Eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PriorityQueueItem {{ priority: {:?} }}", self.priority)
    }
}

struct ReversePriorityQueueItem<T,U> where T : Ord + PartialOrd + Eq{
    priority : T,
    item : U
}

impl<T,U> ReversePriorityQueueItem<T,U> where T : Ord + PartialOrd + Eq{
    fn new(priority : T, item : U) -> Self where T : Ord + PartialOrd + Eq{
        ReversePriorityQueueItem {
            priority,
            item
        }
    }
}

impl<T,U> Ord for ReversePriorityQueueItem<T,U> where T : Ord + PartialOrd + Eq{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T,U> PartialOrd for ReversePriorityQueueItem<T,U> where T : Ord + PartialOrd + Eq{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority).and_then(|x|Some(x.reverse()))
    }
}

impl<T,U> PartialEq for ReversePriorityQueueItem<T,U> where T : Ord + PartialOrd + Eq{
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T,U> Eq for ReversePriorityQueueItem<T,U> where T  : Ord + PartialOrd + Eq{
}

impl<T: fmt::Debug, U> fmt::Debug for ReversePriorityQueueItem<T, U> where T  : Ord + PartialOrd + Eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReversePriorityQueueItem {{ priority: {:?} }}", self.priority)
    }
}