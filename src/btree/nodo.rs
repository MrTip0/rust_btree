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

macro_rules! new_nodo {
    ( $v: expr ) => {
        Next::Nodo(Rc::new(RefCell::new(Nodo::new($v))))
    };
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
                        self.sx = new_nodo!(val);
                    }
                }
            } else {
                match &self.dx {
                    Next::Nodo(n) => n.borrow_mut().add(val),
                    Next::None => {
                        self.dx = new_nodo!(val);
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

    pub(super) fn insert_balanced(&mut self, v: &[i32]) {
        let l =  v.len();
        if l > 0 {
            if l == 1 {
                self.val = v[0];
            } else if l == 2 {
                self.val = v[1];
                self.sx =  new_nodo!(v[0]);
            } else if l == 3 {
                self.sx = new_nodo!(v[0]);
                self.val = v[1];
                self.dx = new_nodo!(v[2]);
            } else {
                let mid = l / 2;
                self.sx = new_nodo!(0);
                match &self.sx {
                    &Next::None => (),
                    Next::Nodo(n) => n.borrow_mut().insert_balanced(&v[..mid])
                }
                self.val = v[mid];
                self.dx = new_nodo!(0);
                match &self.dx {
                    &Next::None => (),
                    Next::Nodo(n) => n.borrow_mut().insert_balanced(&v[mid + 1..])
                }
            }
        }
    }
}