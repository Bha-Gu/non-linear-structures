use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone)]
struct Node<T>
where
    T: Clone + PartialEq + Debug,
{
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "\nL({0:?}): {1:?}\n        V: {0:?} \nR({0:?}): {2:?}\n",
            self.value,
            self.left.clone().map(|a| a.borrow().clone()),
            self.right.clone().map(|a| a.borrow().clone())
        )
    }
}

impl<T: Clone + PartialEq + Debug> Node<T> {
    fn from_vec_dfs(vec: &[T]) -> Self {
        let len = vec.len();
        let mid_idx = if len % 2 == 0 { len / 2 } else { (len - 1) / 2 };
        let value = vec[0].to_owned();
        let left_arr = &vec[1..=mid_idx];
        let right_arr = &vec[mid_idx + 1..];
        let left = if left_arr.is_empty() {
            None
        } else {
            Some(Rc::new(RefCell::new(Self::from_vec_dfs(left_arr))))
        };
        let right = if right_arr.is_empty() {
            None
        } else {
            Some(Rc::new(RefCell::new(Self::from_vec_dfs(right_arr))))
        };
        Self { value, left, right }
    }

    fn dfs(&self) {
        if let Some(left) = self.left.clone() {
            print!("(");
            Self::dfs(&left.borrow().clone());
            print!("<-");
        }
        print!("{:?}", self.value.clone());
        if let Some(right) = self.right.clone() {
            print!("->");
            Self::dfs(&right.borrow().clone());
            print!(")");
        }
    }
}

pub struct BinaryTree<T>
where
    T: Clone + PartialEq + Debug,
{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq + Debug> Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.root.clone().map(|a| a.borrow().clone()))
    }
}

impl<T: Clone + PartialEq + Debug> BinaryTree<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub fn from_vec_dfs(vec: &[T]) -> Self {
        let len = vec.len();
        if len == 0 {
            Self::new()
        } else {
            let root = Some(Rc::new(RefCell::new(Node::from_vec_dfs(vec))));
            Self { root }
        }
    }

    pub fn dfs(&self) {
        if let Some(root) = self.root.clone() {
            Node::dfs(&root.borrow().clone());
            println!();
        } else {
            println!("Tree is Empty");
        };
    }
}
