/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        
        // 否则，调用根节点的insert方法进行插入
        self.root.as_mut().unwrap().insert(value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut current = &self.root;
        
        // 遍历树直到找到值或到达叶子节点
        while let Some(node) = current {
            if value == node.value {
                // 找到值，返回true
                return true;
            } else if value < node.value {
                // 值小于当前节点，向左子树搜索
                current = &node.left;
            } else {
                // 值大于当前节点，向右子树搜索
                current = &node.right;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            // 值小于当前节点，插入左子树
            if self.left.is_none() {
                // 左子树为空，直接创建新节点
                self.left = Some(Box::new(TreeNode::new(value)));
            } else {
                // 左子树不为空，递归插入左子树
                self.left.as_mut().unwrap().insert(value);
            }
        } else if value > self.value {
            // 值大于当前节点，插入右子树
            if self.right.is_none() {
                // 右子树为空，直接创建新节点
                self.right = Some(Box::new(TreeNode::new(value)));
            } else {
                // 右子树不为空，递归插入右子树
                self.right.as_mut().unwrap().insert(value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


