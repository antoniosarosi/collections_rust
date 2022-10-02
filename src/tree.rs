use std::ptr::NonNull;

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

type Link<T> = Option<NonNull<Node<T>>>;

enum DescendingFrom {
    Left,
    Right,
}

pub struct Tree<T> {
    root: Link<T>,
    size: usize,
}

impl<T> Node<T> {
    unsafe fn new(value: T, right: Link<T>, left: Link<T>) -> NonNull<Node<T>> {
        NonNull::new_unchecked(Box::into_raw(Box::new(Node { right, left, value })))
    }
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Traverses the tree until the place where the new value should be
    /// inserted is found. If the value cannot be inserted because it is
    /// already present the function returns false, otherwise it creates a new
    /// node with the value and returns true.
    unsafe fn insert_value(
        &mut self,
        current: Link<T>,
        parent: Link<T>,
        descending_from: DescendingFrom,
        value: T,
    ) -> bool {
        if let Some(node) = current {
            // If the current node is not `None` we compare the new value to
            // the value stored in this node. If it's equal, the value  is
            // already there so return false, otherwise keep searching.
            if value < (*node.as_ptr()).value {
                self.insert_value((*node.as_ptr()).left, current, DescendingFrom::Left, value)
            } else if value > (*node.as_ptr()).value {
                self.insert_value(
                    (*node.as_ptr()).right,
                    current,
                    DescendingFrom::Right,
                    value,
                )
            } else {
                false
            }
        } else {
            // Allocate the new node and make the parent point to it.
            let new_node = Some(Node::new(value, None, None));

            if let Some(node) = parent {
                match descending_from {
                    DescendingFrom::Left => (*node.as_ptr()).left = new_node,
                    DescendingFrom::Right => (*node.as_ptr()).right = new_node,
                }
            } else {
                self.root = new_node;
            }

            self.size += 1;

            true
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        unsafe { self.insert_value(self.root, None, DescendingFrom::Left, value) }
    }
}

// TODO: Implement Drop for Tree. Currently it leask memory.

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn test_size() {
        let mut tree = Tree::new();
        tree.insert(1);
        assert_eq!(tree.size(), 1);
        tree.insert(2);
        assert_eq!(tree.size(), 2);
        tree.insert(3);
        assert_eq!(tree.size(), 3);
    }
}
