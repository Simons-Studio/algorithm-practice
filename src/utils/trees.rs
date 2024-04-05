pub struct BinarySearchTree {
    value: i32,
    left: Option<Box<BinarySearchTree>>,
    right: Option<Box<BinarySearchTree>>
}
impl BinarySearchTree {
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

    pub fn from_array(values: Vec<i32>) -> Option<BinarySearchTree> {
        let mut sorted = values.clone();
        sorted.sort();
        BinarySearchTree::from_sorted_array(sorted)
    }

    fn from_sorted_array(values: Vec<i32>) -> Option<BinarySearchTree> {
        if values.is_empty() {
            return None;
        }

        let middle = values.len() / 2;
        let mid_value = values[middle];
        let left_values = values[0..middle].to_vec();
        let right_values = values[middle+1..].to_vec();

        let left_tree = BinarySearchTree::from_sorted_array(left_values);
        let right_tree = BinarySearchTree::from_sorted_array(right_values);

        let tree = BinarySearchTree {
            value: mid_value,
            left: BinarySearchTree::box_tree(left_tree),
            right: BinarySearchTree::box_tree(right_tree)
        };

        Some(tree)
    }

    // pub fn insert_max(&mut self, value: i32) {}

    pub fn leaf(value: i32) -> BinarySearchTree {
        BinarySearchTree { value, left: None, right: None }
    }

    fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }
        let mut right_branch = *self.right.take().unwrap();
        let transfered_branch = right_branch.left;
        let new_left: BinarySearchTree = BinarySearchTree {
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
        let new_right: BinarySearchTree = BinarySearchTree {
            value: self.value,
            left: transfered_branch,
            right: self.left.take(),
        };
        left_branch.right = Some(Box::new(new_right));

        self.value = left_branch.value;
        self.left = left_branch.left;
        self.right = left_branch.right;
    }

    fn count(&self) -> i32 {
        let mut count = 1;
        if let Some(left) = &self.left {
            count += left.count();
        }
        if let Some(right) = &self.right {
            count += right.count();
        }
        count
    }

    // fn shift_left(&mut self) {}

    fn has_branches(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    fn left_has_branches(&self) -> bool {
        if let Some(left) = &self.left {
            left.left.is_some() && left.right.is_some()
        } else {
            false
        }
    }

    fn box_tree(opt_tree: Option<BinarySearchTree>) -> Option<Box<BinarySearchTree>> {
        if let Some(tree) = opt_tree {
            Some(Box::new(tree))    
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn test_the_middle() {
        let middle = (0..1).len() / 2;
        assert_eq!(middle, 0);

        let middle = (0..11).len() / 2;
        assert_eq!(middle, 5);
    }

    #[test]
    fn test_tree_from_array() {
        let test_str = String::from("[[[0]1[2]]3[[4]5[6]]]");
        let tree_str = BinarySearchTree::from_sorted_array((0..7).collect()).unwrap().to_string();
        assert_eq!(test_str, tree_str);
    }
}