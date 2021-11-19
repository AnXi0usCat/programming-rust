
/// An ordered collection of T's
enum BinaryTree<T> {
    Empty,
    NotEmpty(Box<TreeNode<T>>)
}

/// A part of a binary tree
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NotEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                }))
            }
            BinaryTree::NotEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercurey");
    tree.add("Venus")
}
