use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Nodo {
    sx: Next,
    dx: Next,
    val: i32
}

#[derive(Debug)]
pub enum Next {
    None,
    Nodo(Rc<RefCell<Nodo>>)
}

impl Nodo {

    pub fn new(val: i32) -> Self {
        Nodo{
            val,
            sx: Next::None,
            dx: Next::None
        }
    }

    pub fn find(&self, val: i32) -> bool {
        if val == self.val {
            return true;
        } else {
            if val < self.val {
                match &self.sx {
                    Next::None => return false,
                    Next::Nodo(n) => return n.borrow().find(val)
                };
            } else {
                match &self.dx {
                    Next::None => return false,
                    Next::Nodo(n) => return n.borrow().find(val)
                };
            }
        }
    }

    pub fn add(&mut self, val: i32) {
        if val != self.val {
            if val < self.val {
                match &self.sx {
                    Next::Nodo(n) => n.borrow_mut().add(val),
                    Next::None => {
                        self.sx = Next::Nodo(Rc::new(RefCell::new(Nodo::new(val))));
                    }
                }
            } else {
                match &self.dx {
                    Next::Nodo(n) => n.borrow_mut().add(val),
                    Next::None => {
                        self.dx = Next::Nodo(Rc::new(RefCell::new(Nodo::new(val))));
                    }
                }
            }
        }
    }

    pub(super) fn to_vec(&self, mut act: Vec<i32>) -> Vec<i32> {
        match &self.sx {
            Next::None => (),
            Next::Nodo(n) => {act = n.borrow().to_vec(act)}
        };

        act.append(&mut vec![self.val]);

        match &self.dx {
            Next::None => (),
            Next::Nodo(n) => {act = n.borrow().to_vec(act)}
        };

        return act;
    }
}
