trait MyTrait {}

#[derive(Clone)]
struct MyStruct1 (u8);
struct MyStruct2 (u8);

impl MyTrait for MyStruct1 {}
impl MyTrait for MyStruct2 {}

fn main() {
    let my_struct_1 = MyStruct1(1);
    let my_struct_2 = MyStruct2(1);

    let mut my_collection: Vec<MyStruct1> = vec![];

    my_collection.push(my_struct_1.clone());
    //my_collection.push(my_struct_2);

    let mut my_collection: Vec<Box<dyn MyTrait>> = vec![];

    my_collection.push(Box::new(my_struct_1));
    my_collection.push(Box::new(my_struct_2));
}
