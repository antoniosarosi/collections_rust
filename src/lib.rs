struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Self>>) -> Self {
        Node { value, next }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: u32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn append(&mut self, value: T) {
        let mut node = Box::new(Node::new(value, None));

        let new_tail: *mut Node<T> = &mut *node;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(node);
            }
        } else {
            self.head = Some(node);
        }

        self.tail = new_tail;

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            self.len -= 1;

            head.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);

        // Populate list
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.len(), 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.len(), 1);

        // append some more just to make sure nothing's corrupted
        list.append(4);
        list.append(5);
        assert_eq!(list.len(), 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.len(), 1);

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        // Check the exhaustion case fixed the pointer right
        list.append(6);
        list.append(7);
        assert_eq!(list.len(), 2);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }
}
