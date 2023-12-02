use std::cmp::Ordering;

/// # 20.3 Exercise: Binary Tree
///
/// A binary tree is a tree-type data structure where every node has two
/// children (left and right). We will create a tree where each node stores
/// a value. For a given node N, all nodes in a N’s left subtree contain
/// smaller values, and all nodes in N’s right subtree will contain larger
/// values.
///
/// Implement an iterator over a binary tree that returns the values in
/// order.
///

#[derive(Debug)]
struct BinaryTreeNode<T: Ord + Copy> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.

type TreeNode<T> = Box<BinaryTreeNode<T>>;

#[derive(Debug)]
pub struct BinaryTree<T: Ord + Copy>(Option<TreeNode<T>>);

// Implement `new`, `insert`, and `has`.

impl<T: Ord + Copy> BinaryTree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn has(&self, value: T) -> bool {
        match &self.0 {
            Some(node) => match node.value.cmp(&value) {
                Ordering::Less => node.right.has(value),
                Ordering::Equal => true,
                Ordering::Greater => node.left.has(value),
            },
            _ => false,
        }
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            Some(node) => match node.value.cmp(&value) {
                Ordering::Less => node.right.insert(value),
                Ordering::Equal => {}
                Ordering::Greater => node.left.insert(value),
            },
            _ => {
                self.0 = Some(Box::new(BinaryTreeNode {
                    value,
                    left: BinaryTree::new(),
                    right: BinaryTree::new(),
                }));
            }
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => node.left.len() + node.right.len() + 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(i as i32)).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(50));
    }
}

pub fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(7);
    tree.insert(3);
    tree.insert(15);

    println!("{:?}", tree);
}
