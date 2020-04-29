pub trait Plugin {
    fn go(&self) -> bool;
}

struct MyPlugin {}
struct MyOtherPlugin {}

impl MyPlugin {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for MyPlugin {
    fn go(&self) -> bool {
        true
    }
}

impl MyOtherPlugin {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for MyOtherPlugin {
    fn go(&self) -> bool {
        false
    }
}

fn main() {
    let my_plugin = MyPlugin::new();            // instantiate a plugin
    let result = my_plugin.go();                // call the trait method
    let my_other_plugin = MyOtherPlugin::new(); // instantiate a plugin
    let result2 = my_other_plugin.go();         // call the trait method
    println!("{}, {}", result, result2);
}

