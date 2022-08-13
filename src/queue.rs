use std::fmt::Display;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: *mut Node<T>) -> Self {
        Node { value, next }
    }
}

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: u32,
}

pub struct IntoIter<T>(Queue<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn append(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node::new(value, std::ptr::null_mut())));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = node;
            }
        } else {
            self.head = node;
        }

        self.tail = node;

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let head = Box::from_raw(self.head);

            self.head = head.next;

            if self.head.is_null() {
                self.tail = std::ptr::null_mut();
            }

            self.len -= 1;

            Some(head.value)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.value) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.value
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.value
            })
        }
    }
}

impl<T: Display> Display for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        if self.len() == 1 {
            return write!(f, "[{}]", self.peek().unwrap());
        }

        let mut iter = self.iter();
        write!(f, "[{}", iter.next().unwrap())?;
        for value in iter {
            write!(f, ", {}", value)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty list behaves right
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.pop(), None);

        // Populate list
        queue.append(1);
        queue.append(2);
        queue.append(3);
        assert_eq!(queue.len(), 3);

        // Check normal removal
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.len(), 1);

        // append some more just to make sure nothing's corrupted
        queue.append(4);
        queue.append(5);
        assert_eq!(queue.len(), 3);

        // Check normal removal
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.len(), 1);

        // Check exhaustion
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);

        // Check the exhaustion case fixed the pointer right
        queue.append(6);
        queue.append(7);
        assert_eq!(queue.len(), 2);

        // Check normal removal
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), Some(7));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn into_iter() {
        let mut queue = Queue::new();
        queue.append(1);
        queue.append(2);
        queue.append(3);

        let mut iter = queue.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut queue = Queue::new();
        queue.append(1);
        queue.append(2);
        queue.append(3);

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut queue = Queue::new();
        queue.append(1);
        queue.append(2);
        queue.append(3);

        let mut iter = queue.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn display() {
        let mut queue = Queue::<i32>::new();
        assert_eq!(format!("{queue}"), "[]");

        queue.append(1);
        assert_eq!(format!("{queue}"), "[1]");

        queue.append(2);
        assert_eq!(format!("{queue}"), "[1, 2]");

        for i in 3..6 {
            queue.append(i);
        }

        assert_eq!(format!("{queue}"), "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn miri_food() {
        let mut queue = Queue::new();

        queue.append(1);
        queue.append(2);
        queue.append(3);

        assert!(queue.pop() == Some(1));
        queue.append(4);
        assert!(queue.pop() == Some(2));
        queue.append(5);

        assert!(queue.peek() == Some(&3));
        queue.append(6);
        queue.peek_mut().map(|x| *x *= 10);
        assert!(queue.peek() == Some(&30));
        assert!(queue.pop() == Some(30));

        for elem in queue.iter_mut() {
            *elem *= 100;
        }

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(queue.pop() == Some(400));
        queue.peek_mut().map(|x| *x *= 10);
        assert!(queue.peek() == Some(&5000));
        queue.append(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
