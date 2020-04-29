// Rc<T>
// Shared, usually immutable, ownership of the same data within a single thread

use std::rc::Rc;

struct Gadget {
    owner: Rc<String>,
}

fn main() {
    let gadget_owner: Rc<String> = Rc::new(String::from("Gadget Man"));

    let gadget1 = Gadget {
        owner: Rc::clone(&gadget_owner),
    };

    let gadget2 = Gadget {
        owner: Rc::clone(&gadget_owner),
    };

    drop(gadget_owner);

    println!("Gadget 1 owned by {}", gadget1.owner);
    println!("Gadget 2 owned by {}", gadget2.owner);
}

// https://doc.rust-lang.org/book/ch15-04-rc.html
// https://doc.rust-lang.org/std/rc/index.html
// https://doc.rust-lang.org/1.5.0/book/choosing-your-guarantees.html

// Weak<T> is similar to Rc<T> but doesn't guarantee ownership, so
// dereferences to Option<T>
