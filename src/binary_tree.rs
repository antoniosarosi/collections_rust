use std::{marker, ptr};

/// Binary tree node.
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

/// Rusty pointer to a node.
type Link<T> = Option<ptr::NonNull<Node<T>>>;

/// Main binary tree struct.
pub struct BinaryTree<T> {
    root: Link<T>,
    size: usize,
    value_inserted: bool,
    value_removed: bool,
    _marker: marker::PhantomData<T>,
}

impl<T> Node<T> {
    /// Allocates a new node and returns a `ptr::NonNull` to the node.
    unsafe fn new_non_null(value: T, right: Link<T>, left: Link<T>) -> ptr::NonNull<Node<T>> {
        ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node { right, left, value })))
    }
}

impl<T: Ord> BinaryTree<T> {
    /// Creates a new binary tree. Doesn't allocate memory until first value
    /// is inserted.
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
            value_inserted: false,
            value_removed: false,
            _marker: marker::PhantomData,
        }
    }

    /// Returns the current number of elements in the tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(1);
    /// assert_eq!(tree.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns `true` if the tree contains no values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    ///
    /// tree.remove(&1);
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Recursive function for inserting nodes in the tree. The funciton allways
    /// returns a node to the caller, either the current node or the new inserted
    /// node.
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

    /// Adds the given `value` to the tree and returns `true` unless it is
    /// already present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(1);
    /// assert!(tree.contains(&1));
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        unsafe {
            self.root = self.insert_recursively(self.root, value);
        }

        self.value_inserted
    }

    /// Returns `true` if the node that contains `value` can be located.
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

    /// Returns `true` if `value` is present in the tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(1);
    /// assert!(tree.contains(&1));
    /// assert!(!tree.contains(&2));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        unsafe { self.search(self.root, value) }
    }

    /// Returns a pointer to the parent node of the node that contains the
    /// minimum value in the given subtree. Used for searching inorder successors.
    unsafe fn min_value_parent_node(&self, node: ptr::NonNull<Node<T>>) -> Link<T> {
        match (*node.as_ptr()).left {
            None => None,

            Some(subnode) => match (*subnode.as_ptr()).left {
                None => Some(node),
                Some(_) => self.min_value_parent_node(subnode),
            },
        }
    }

    /// Performs the binary tree node removal algorithm:
    ///
    /// - Find the node that contains `value`.
    ///
    /// - If the node only has one child, deallocate the node and make the parent
    /// point to the child.
    ///
    /// - If the node has no children just make the parent point to any of its
    /// non-existent children (set the `Link<T>` to `None`).
    ///
    /// - If the node has two children, locate the inorder successor of the
    /// current node in the right subtree, swap the values, deallocate the
    /// successor and make the successor parent point to `None`. The case where
    /// the inorder successor parent is the root node has to be considered.
    ///
    /// Just like `insert_recursively`, a `Link<T>` will allways be returned
    /// to the caller. This simplifies the amount of cases we have to deal with.
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

        // Node has two children
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

    /// Removes the `value` from the tree and returns `true` unless the `value`
    /// is not present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    ///
    /// tree.remove(&2);
    ///
    /// assert!(!tree.contains(&2));
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        unsafe {
            self.root = self.remove_recursively(self.root, value);
        }

        self.value_removed
    }
}

impl<T> BinaryTree<T> {
    /// Drop the left subtree, drop the right subtree and then drop the root.
    unsafe fn drop_recursively(&mut self, current: Link<T>) {
        if let Some(node) = current {
            self.drop_recursively((*node.as_ptr()).left);
            self.drop_recursively((*node.as_ptr()).right);
            drop(Box::from_raw(node.as_ptr()));
        }
    }
}

impl<T> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        unsafe { self.drop_recursively(self.root) }
    }
}

pub struct Iter<'a, T> {
    values: Vec<&'a T>,
    current_index: usize,
}

impl<T> BinaryTree<T> {
    /// Fills `values` vector using inorder traversal.
    unsafe fn push_values_inorder(&self, current: Link<T>, values: &mut Vec<&T>) {
        if let Some(node) = current {
            self.push_values_inorder((*node.as_ptr()).left, values);
            values.push(&(*node.as_ptr()).value);
            self.push_values_inorder((*node.as_ptr()).right, values);
        }
    }

    /// Returns an iterator over the values contained in the tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_rust::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    ///
    /// tree.insert(3);
    /// tree.insert(1);
    /// tree.insert(2);
    ///
    /// let mut expected = 1;
    /// for value in tree.iter() {
    ///     assert_eq!(value, &expected);
    ///     expected += 1;
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        let mut values = Vec::with_capacity(self.size);

        unsafe {
            self.push_values_inorder(self.root, &mut values);
        }

        Iter {
            values,
            current_index: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.values.len() {
            return None;
        }

        let value = self.values[self.current_index];

        self.current_index += 1;

        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.values.len() - self.current_index;

        (remaining, Some(remaining))
    }
}

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

    #[test]
    fn test_iter() {
        let mut values = tree_values();

        let mut tree = BinaryTree::new();

        for value in values.iter() {
            tree.insert(*value);
        }

        let mut iter = tree.iter();

        values.sort();

        for value in values.iter() {
            let tree_value = iter.next();
            assert!(tree_value.is_some());
            assert_eq!(value, tree_value.unwrap());
        }

        assert!(iter.next().is_none());
    }
}
