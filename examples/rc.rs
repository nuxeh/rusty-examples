// Rc<T>
// Shared, usually immutable, ownership of the same data within a single thread

struct Tree {

}

fn main() {

}

// https://doc.rust-lang.org/book/ch15-04-rc.html
// https://doc.rust-lang.org/std/rc/index.html

// Weak<T> is similar to Rc<T> but doesn't guarantee ownership, so
// dereferences to Option<T>
