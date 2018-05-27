mod priority_queues;
pub use self::priority_queues::{PriorityQueue,ReversePriorityQueue};

#[cfg(test)]
mod tests {
    use priority_queues::{PriorityQueue, ReversePriorityQueue};

    fn new_test_priority_queue<'a>() -> PriorityQueue<i32, &'a str>{
        let mut q = PriorityQueue::new();

        q.push(2, "a");
        q.push(-52, "b");
        q.push(9, "c");

        q
    }

    fn new_test_reverse_priority_queue<'a>() -> ReversePriorityQueue<i32, &'a str>{
        let mut q = ReversePriorityQueue::new();

        q.push(2, "a");
        q.push(-52, "b");
        q.push(9, "c");

        q
    }

    #[test]
    fn test_priority_queue_pop(){
        let mut q = new_test_priority_queue();

        assert_eq!(q.pop().unwrap(),(9, "c"));
        assert_eq!(q.pop().unwrap(),(2, "a"));
        assert_eq!(q.pop().unwrap(),(-52, "b"));
        assert_eq!(q.pop(),None);
    }

    #[test]
    fn test_reverse_priority_queue_pop(){
        let mut q = new_test_reverse_priority_queue();

        assert_eq!(q.pop().unwrap(),(-52, "b"));
        assert_eq!(q.pop().unwrap(),(2, "a"));
        assert_eq!(q.pop().unwrap(),(9, "c"));
        assert_eq!(q.pop(),None);
    }

    #[test]
    fn test_priority_queue_peek(){
        let mut q = new_test_priority_queue();

        assert_eq!(q.peek().unwrap(),(&9, &"c"));
        assert_eq!(q.pop().unwrap(),(9, "c"));
        assert_eq!(q.peek().unwrap(),(&2, &"a"));
        assert_eq!(q.pop().unwrap(),(2, "a"));
        assert_eq!(q.peek().unwrap(),(&-52, &"b"));
        assert_eq!(q.peek().unwrap(),(&-52, &"b"));
        assert_eq!(q.pop().unwrap(),(-52, "b"));
        assert_eq!(q.peek(),None);
        assert_eq!(q.pop(),None);
    }

    #[test]
    fn test_reverse_priority_queue_peek(){
        let mut q = new_test_reverse_priority_queue();

        assert_eq!(q.peek().unwrap(),(&-52, &"b"));
        assert_eq!(q.pop().unwrap(),(-52, "b"));
        assert_eq!(q.peek().unwrap(),(&2, &"a"));
        assert_eq!(q.pop().unwrap(),(2, "a"));
        assert_eq!(q.peek().unwrap(),(&9, &"c"));
        assert_eq!(q.peek().unwrap(),(&9, &"c"));
        assert_eq!(q.pop().unwrap(),(9, "c"));
        assert_eq!(q.peek(),None);
        assert_eq!(q.pop(),None);
    }

}


