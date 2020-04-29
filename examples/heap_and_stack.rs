fn func() -> String{
    let _baz = 42; // _baz is stored in the stack frame for func()

    // If we create a string, the storage for it is held on the heap
    let qux = String::from("infinitely long string");

    // If we return qux, we are returning a copy of it, which still owns the
    // associated heap-allocated memory (the String struct contains a pointer)
    qux

    // _baz is dropped when it goes out of scope
}


fn main() {
    let _foo = 8; // _foo is stored in the stack frame for main()

    // If we create a string, the storage for it is held on the heap
    let _bar = String::from("infinitely long string");

    let string = func();
    println!("{}", string);

    // _foo is dropped when it goes out of scope
    // _bar's heap memory is deallocated when the String object is dropped
    // _string's heap memory is deallocated when the String object is dropped
}
