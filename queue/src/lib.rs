pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

#[test]
fn test_push_pop() {
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('2');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('2'));
    assert_eq!(q.pop(), None);
}

#[test]
fn test_is_empty() {
    let mut q = Queue::new();

    assert!(q.is_empty());
    q.push('0');
    assert!(!q.is_empty());
}

#[test]
fn test_split() {
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('2');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['1']);
    assert_eq!(younger, vec!['2']);
}
