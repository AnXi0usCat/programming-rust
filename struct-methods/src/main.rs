pub mod queue;
pub mod generic_queue;
use crate::queue::Queue;
use crate::generic_queue::GenericQueue;

fn main() {
    let mut q = Queue::new();

    q.push('0');
    q.push('1');

    assert_eq!(q.pop(), Some('0'));

    q.push('=');

    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('='));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('=');
    assert!(!q.is_empty());
    q.pop();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('=');

    let (older, newer) = q.split();
    assert_eq!(older, vec!['1']);
    assert_eq!(newer, vec!['=']);

    // generic queue
    let mut q_gen = GenericQueue::<i32>::new();

    q_gen.push(0);
    q_gen.push(1);
    assert_eq!(q_gen.pop(), Some(0));
}
