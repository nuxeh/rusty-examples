pub trait Plugin {
    fn go(&self) -> bool;

    fn ping(&self) -> bool {
        true
    }
}

struct MyPlugin {}
struct MyOtherPlugin {}

impl Plugin for MyPlugin {
    fn go(&self) -> bool {
        true
    }
}

impl Plugin for MyOtherPlugin {
    fn go(&self) -> bool {
        false
    }
}

fn main() {
    let my_plugin = MyPlugin {};            // instantiate a plugin
    let my_other_plugin = MyOtherPlugin {}; // instantiate a plugin

    let result = my_plugin.go();            // call the trait method
    let result2 = my_other_plugin.go();     // call the trait method

    println!("{}, {}", result, result2);

    let result = my_plugin.ping();            // call the trait method
    let result2 = my_other_plugin.ping();     // call the trait method

    println!("{}, {}", result, result2);
}
