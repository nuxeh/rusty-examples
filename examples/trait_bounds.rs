use std::path::{Path, PathBuf};

fn take_ref(r: impl AsRef<Path>) {
    println!("{:?}", r.as_ref())
}

fn take_ref_2<T: AsRef<Path>>(r: T) {
    println!("{:?}", r.as_ref())
}

fn take_copy(_c: impl Copy) {}

fn take_copy_2<T: Copy>(_c: T) {}

fn main() {
    let p = PathBuf::from("hi");
    let s = String::from("hi");

    take_ref("hi");
    take_ref_2("hi");

    take_ref(&p);
    take_ref_2(&p);

    take_ref(&s);
    take_ref_2(&s);

    //take_ref(1);
    //take_ref_2(1);

    take_copy(1);
    take_copy_2(1);

    //take_copy(String::from("hehheh i can't be copied"));
    //take_copy_2(String::from("hehheh i can't be copied"));
}
