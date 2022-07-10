enum BinaryTree<T> {
    Empty,
    NotEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
