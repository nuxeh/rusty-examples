trait MyTrait {}

struct MyStruct1 (u8);
struct MyStruct2 (u8);

impl MyTrait for MyStruct1 {}

fn only_my_trait<T: MyTrait>(_input: &T) {
    println!("ok")
}

fn main() {
    let my_struct_1 = MyStruct1(1);
    let my_struct_2 = MyStruct2(1);

    only_my_trait(&my_struct_1);
    // only_my_trait(&my_struct_2);
}
