mod btree;

use std::io::{stdin, stdout, Write};

use crate::btree::btree::BTree;


macro_rules! presente {
    ( $v: expr ) => {
        match $v {
            true => println!("Presente"),
            false => println!("Assente")
        }
    };
}


macro_rules! print_vec {
    ( $v: expr ) => {
        let l = $v.len();
        if l > 0 {
            print!("[");
            let mut i = 0;
            while i < l - 1 {
                print!("{}, ", $v[i]);
                i = i + 1;
            }
            println!("{}]", $v[l - 1]);
        } else {
            println!("[]");
        }
    };
}

fn main() {
    let mut tree = BTree::empty();

    loop {
        stampa("Inserisci un numero: ");
        let n = numero();
        match n {
            Some(num) => tree.add(num),
            None => { println!("Fine numeri"); break; }
        };
    }

    tree.balance();

    let asv = Vec::from(tree.clone());

    print_vec!(asv);

    loop {
        stampa("Inserisci un numero da cercare: ");
        let n = numero();
        match n {
            Some(num) => presente!(tree.find(num)),
            None => {println!("Fine ricerche"); break;}
        };
    }
}

fn numero() -> Option<i32> {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("error while reading input");

    let n = input.trim().parse::<i32>();

    match n {
        Ok(num) => Some(num),
        Err(_) => None
    }
}

fn stampa(s: &str) {
    print!("{}", s);
    stdout().flush().unwrap()
}
