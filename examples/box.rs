// Box<T>
// Allows indirection onto the heap

#[allow(dead_code)]
struct Nested {
//    nested: Nested,
}

#[allow(dead_code)]
struct NestedBox {
    nested_box: Option<Box<NestedBox>>,
}

fn main() {
    let foo = NestedBox { nested_box: None };
    let _bar = NestedBox { nested_box: Some(Box::new(foo)) };

    // But what happens when you try to clone a Box?
    let baz = Box::new(1);
    let qux = baz.clone();
    // It's ok, but you will have duplicated the memory for the contents.
}

// https://doc.rust-lang.org/book/ch15-01-box.html
// https://doc.rust-lang.org/1.5.0/book/choosing-your-guarantees.html
