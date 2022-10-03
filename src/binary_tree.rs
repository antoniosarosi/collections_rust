use std::{marker, ptr};

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

type Link<T> = Option<ptr::NonNull<Node<T>>>;

pub struct BinaryTree<T> {
    root: Link<T>,
    size: usize,
    value_inserted: bool,
    value_removed: bool,
    _marker: marker::PhantomData<T>,
}

impl<T> Node<T> {
    unsafe fn new_non_null(value: T, right: Link<T>, left: Link<T>) -> ptr::NonNull<Node<T>> {
        ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node { right, left, value })))
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
            value_inserted: false,
            value_removed: false,
            _marker: marker::PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    unsafe fn insert_recursively(&mut self, mut current: Link<T>, value: T) -> Link<T> {
        if let Some(node) = current {
            if value < (*node.as_ptr()).value {
                (*node.as_ptr()).left = self.insert_recursively((*node.as_ptr()).left, value);
            } else if value > (*node.as_ptr()).value {
                (*node.as_ptr()).right = self.insert_recursively((*node.as_ptr()).right, value);
            } else {
                self.value_inserted = false;
            }
        } else {
            current = Some(Node::new_non_null(value, None, None));
            self.value_inserted = true;
            self.size += 1;
        }

        current
    }

    pub fn insert(&mut self, value: T) -> bool {
        unsafe {
            self.root = self.insert_recursively(self.root, value);
        }

        self.value_inserted
    }

    unsafe fn search(&self, current: Link<T>, value: &T) -> bool {
        match current {
            None => false,

            Some(node) => {
                if value < &(*node.as_ptr()).value {
                    self.search((*node.as_ptr()).left, value)
                } else if value > &(*node.as_ptr()).value {
                    self.search((*node.as_ptr()).right, value)
                } else {
                    true
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        unsafe { self.search(self.root, value) }
    }

    unsafe fn min_value_parent_node(&self, node: ptr::NonNull<Node<T>>) -> Link<T> {
        match (*node.as_ptr()).left {
            None => None,

            Some(subnode) => match (*subnode.as_ptr()).left {
                None => Some(node),
                Some(_) => self.min_value_parent_node(subnode),
            },
        }
    }

    unsafe fn remove_recursively(&mut self, current: Link<T>, value: &T) -> Link<T> {
        // Not found
        if current.is_none() {
            self.value_removed = false;
            return None;
        }

        // Search node
        let node = current.unwrap();

        if value < &(*node.as_ptr()).value {
            (*node.as_ptr()).left = self.remove_recursively((*node.as_ptr()).left, value);
            return current;
        }
        if value > &(*node.as_ptr()).value {
            (*node.as_ptr()).right = self.remove_recursively((*node.as_ptr()).right, value);
            return current;
        }

        // Found
        self.value_removed = true;
        self.size -= 1;

        // Node has only one child or none
        let mut replacement_node = None;
        if (*node.as_ptr()).left.is_none() {
            replacement_node = Some((*node.as_ptr()).right);
        } else if (*node.as_ptr()).right.is_none() {
            replacement_node = Some((*node.as_ptr()).left);
        }
        if replacement_node.is_some() {
            drop(Box::from_raw(node.as_ptr()));
            return replacement_node.unwrap();
        }

        // Node has two children, search parent of inorder successor,
        // replace node value with inorder succesor value and drop successor.
        let node_to_be_dropped;

        if let Some(parent) = self.min_value_parent_node((*node.as_ptr()).right.unwrap()) {
            node_to_be_dropped = (*parent.as_ptr()).left.unwrap();
            let left = ptr::read(node_to_be_dropped.as_ptr());
            (*node.as_ptr()).value = left.value;
            (*parent.as_ptr()).left = left.right;
        } else {
            node_to_be_dropped = (*node.as_ptr()).right.unwrap();
            let right = ptr::read(node_to_be_dropped.as_ptr());
            (*node.as_ptr()).value = right.value;
            (*node.as_ptr()).right = right.right;
        }

        drop(Box::from_raw(node_to_be_dropped.as_ptr()));

        current
    }

    pub fn remove(&mut self, value: &T) -> bool {
        unsafe {
            self.root = self.remove_recursively(self.root, value);
        }

        self.value_removed
    }

    // fn to_string(&self, current: Link<T>) -> String {
    //     match current {
    //         None => String::from(""),

    //         Some(node) => unsafe {
    //             let mut str = String::new();
    //             let left = self.to_string((*node.as_ptr()).left);
    //             let right = self.to_string((*node.as_ptr()).right);

    //             if left.len() > 0 {
    //                 str.push_str(left.as_str());
    //                 str.push_str(", ");
    //             }
    //             str.push_str(format!("{:?}", (*node.as_ptr()).value).as_str());
    //             if right.len() > 0 {
    //                 str.push_str(", ");
    //                 str.push_str(right.as_str());
    //             }

    //             str
    //         },
    //     }
    // }
}

// TODO: Implement Drop for Tree. Currently it leaks memory.

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    fn tree_values() -> Vec<i32> {
        vec![20, 10, 30, 5, 15, 16, 14, 25, 21, 35, 40, 50]
    }

    #[test]
    fn test_insert() {
        let numbers = tree_values();

        let mut tree = BinaryTree::new();

        tree.insert(numbers[0]);
        assert!(tree.contains(&numbers[0]));

        tree.insert(numbers[1]);
        assert!(tree.contains(&numbers[1]));

        tree.insert(numbers[2]);
        assert!(tree.contains(&numbers[2]));

        assert_eq!(tree.size(), 3);

        for n in &numbers[3..] {
            tree.insert(*n);
        }

        for n in &numbers {
            assert!(tree.contains(n));
        }

        assert_eq!(tree.size(), numbers.len());
    }

    #[test]
    fn test_remove() {
        let numbers = tree_values();

        let mut tree = BinaryTree::new();

        for n in &numbers {
            tree.insert(*n);
        }

        // Node with no children
        tree.remove(&50);
        assert!(!tree.contains(&50));

        // Node with one child to the right
        tree.remove(&35);
        assert!(!tree.contains(&35));
        assert!(tree.contains(&40));

        // Node with one child to the left
        tree.remove(&25);
        assert!(!tree.contains(&25));
        assert!(tree.contains(&21));

        // Node with two children
        tree.remove(&10);
        assert!(!tree.contains(&10));
        assert!(tree.contains(&5));
        assert!(tree.contains(&15));

        // Root
        tree.remove(&20);
        assert!(!tree.contains(&20));

        // Check remaining values
        assert!(tree.contains(&40));
        assert!(tree.contains(&21));
        assert!(tree.contains(&30));
        assert!(tree.contains(&5));
        assert!(tree.contains(&15));
        assert!(tree.contains(&14));
        assert!(tree.contains(&16));
    }
}
