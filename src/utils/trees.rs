struct BinarySearchTree();
impl BinarySearchTree {
    pub fn insert_max(&mut self, value: i32) {}
}

pub struct BinarySearchTreeNode {
    value: i32,
    left: Option<Box<BinarySearchTreeNode>>,
    right: Option<Box<BinarySearchTreeNode>>
}
impl BinarySearchTreeNode {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        string.push('[');
        if let Some(left) = &self.left {
            let left_str = left.to_string();
            string.push_str(&left_str);
        }

        string.push_str(&self.value.to_string());

        if let Some(right) = &self.right {
            let right_str = right.to_string();
            string.push_str(&right_str);
        }
        string.push(']');

        string
    }

    pub fn insert_max(&mut self, value: i32) {
        self.right = Some(Box::new(BinarySearchTreeNode::leaf(value)));

        
    }

    pub fn leaf(value: i32) -> BinarySearchTreeNode {
        BinarySearchTreeNode { value, left: None, right: None }
    }

    fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }
        let mut right_branch = *self.right.take().unwrap();
        let transfered_branch = right_branch.left;
        let new_left: BinarySearchTreeNode = BinarySearchTreeNode {
            value: self.value,
            left: self.left.take(),
            right: transfered_branch
        };
        right_branch.left = Some(Box::new(new_left));

        self.value = right_branch.value;
        self.left = right_branch.left;
        self.right = right_branch.right;
    }

    fn rotate_right(&mut self) {
        if self.left.is_none() {
            return;
        }
        let mut left_branch = *self.left.take().unwrap();
        let transfered_branch = left_branch.right;
        let new_right: BinarySearchTreeNode = BinarySearchTreeNode {
            value: self.value,
            left: transfered_branch,
            right: self.left.take(),
        };
        left_branch.right = Some(Box::new(new_right));

        self.value = left_branch.value;
        self.left = left_branch.left;
        self.right = left_branch.right;
    }

    // fn left_full(&self) -> bool {

    // }
}
