pub mod queue;
use crate::queue::Queue;

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
}
