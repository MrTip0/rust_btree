use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Nodo<T: Ord + Eq + Clone> {
    sx: Option<Rc<RefCell<Nodo<T>>>>,
    dx: Option<Rc<RefCell<Nodo<T>>>>,
    val: T
}

macro_rules! new_nodo {
    ( $v: expr ) => {
        Some(Rc::new(RefCell::new(Nodo::new($v))))
    };
}

impl<T: Ord + Eq + Clone> Nodo<T> {
    pub fn new(val: T) -> Self {
        Nodo {
            val,
            sx: None,
            dx: None
        }
    }

    pub fn find(&self, val: T) -> bool {
        if val == self.val {
            return true;
        } else {
            if val < self.val {
                match &self.sx {
                    None => return false,
                    Some(n) => return n.borrow().find(val)
                };
            } else {
                match &self.dx {
                    None => return false,
                    Some(n) => return n.borrow().find(val)
                };
            }
        }
    }

    pub fn add(&mut self, val: T) {
        if val != self.val {
            if val < self.val {
                match &self.sx {
                    Some(n) => n.borrow_mut().add(val),
                    None => {
                        self.sx = new_nodo!(val);
                    }
                }
            } else {
                match &self.dx {
                    Some(n) => n.borrow_mut().add(val),
                    None => {
                        self.dx = new_nodo!(val);
                    }
                }
            }
        }
    }

    pub(super) fn to_vec(&self, mut act: Vec<T>) -> Vec<T> {
        match &self.sx {
            None => (),
            Some(n) => {act = n.borrow().to_vec(act)}
        };

        act.append(&mut vec![self.val.clone()]);

        match &self.dx {
            None => (),
            Some(n) => {act = n.borrow().to_vec(act)}
        };

        return act;
    }

    pub(super) fn insert_balanced(&mut self, v: &[T]) {
        let l =  v.len();
        if l > 0 {
            if l == 1 {
                self.val = v[0].clone();
            } else if l == 2 {
                self.val = v[1].clone();
                self.sx =  new_nodo!(v[0].clone());
            } else if l == 3 {
                self.sx = new_nodo!(v[0].clone());
                self.val = v[1].clone();
                self.dx = new_nodo!(v[2].clone());
            } else {
                let mid = l / 2;
                self.sx = new_nodo!(self.val.clone());
                match &self.sx {
                    &None => (),
                    Some(n) => n.borrow_mut().insert_balanced(&v[..mid])
                }
                self.val = v[mid].clone();
                self.dx = new_nodo!(self.val.clone());
                match &self.dx {
                    &None => (),
                    Some(n) => n.borrow_mut().insert_balanced(&v[mid + 1..])
                }
            }
        }
    }
}