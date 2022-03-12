use std::{rc::Rc, cell::RefCell};

use super::nodo::Nodo;

#[derive(Clone, Debug)]
pub struct BTree {
    head: Rc<RefCell<Nodo>>
}

impl BTree {
    pub fn new(val: i32) -> Self {
        BTree {
            head: Rc::new(RefCell::new(Nodo::new(val)))
        }
    }

    pub fn add(&mut self, val: i32) {
        self.head.borrow_mut().add(val)
    }

    pub fn find(&self, val: i32) -> bool {
        self.head.borrow().find(val)
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.head.borrow().to_vec(vec![])
    }

    pub fn balance(&mut self) {
        let asvec = self.to_vec();
        self.head = Rc::new(RefCell::new(Nodo::new(0)));
        self.head.borrow_mut().insert_balanced(&asvec[..]);
    }
}

impl From<BTree> for Vec<i32> {
    fn from(tree: BTree) -> Self {
        tree.to_vec()
    }
}
