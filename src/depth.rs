#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn depth(){
    let mut root = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(9)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(20)));
    root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    println!("Maximum depth of the tree: {}", max_depth(root.as_ref()));
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}