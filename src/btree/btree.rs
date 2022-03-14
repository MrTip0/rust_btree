use std::{rc::Rc, cell::RefCell};

use super::nodo::Nodo;

#[derive(Clone, Debug)]
pub struct BTree<T: Ord + Eq + Clone> {
    head: Option<Rc<RefCell<Nodo<T>>>>
}

impl<T: Ord + Eq + Clone> BTree<T>{
    pub fn new(val: T) -> Self {
        BTree {
            head: Some(Rc::new(RefCell::new(Nodo::new(val))))
        }
    }

    pub fn empty() -> Self {
        BTree {
            head: None
        }
    }

    pub fn add(&mut self, val: T) {
        match &self.head {
            Some(n) => n.borrow_mut().add(val),
            None => self.head = Some(Rc::new(RefCell::new(Nodo::new(val))))
        };
    }

    pub fn find(&self, val: T) -> bool {
        match &self.head {
            Some(n) => n.borrow().find(val),
            None => false
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match &self.head {
            Some(n) => n.borrow().to_vec(vec![]),
            None => vec![]
        }
    }

    pub fn balance(&mut self) {
        let asvec = self.to_vec();
        if asvec.len() == 0 {
            self.head = None;
        } else {
            self.head = Some(Rc::new(RefCell::new(Nodo::new(asvec[0].clone()))));
            match &self.head {
                Some(n) => n.borrow_mut().insert_balanced(&asvec[..]),
                None => ()
            };
        }
    }
}

impl<T: Ord + Eq + Clone> From<BTree<T>> for Vec<T> {
    fn from(tree: BTree<T>) -> Self {
        tree.to_vec()
    }
}

#[macro_export]
macro_rules! btree {
    ( $( $x: tt ),*) => {
        {
            let mut t_tree = BTree::empty();
            $(
                t_tree.add($x);
            )*
            t_tree
        }
    };
}