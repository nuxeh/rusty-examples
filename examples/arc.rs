// Arc<T>
// Shared, usually immutable, ownership of the same data across threads
// Similar to Rc<T>, but thread safe

use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(2e+3 as u64);

    let shared_string = Arc::new(Mutex::new(String::from("hi! ")));
    let shared_string_thread = Arc::clone(&shared_string);

    let _thread = thread::spawn(move || {
        loop {
            thread::sleep(delay);
            let mut string = shared_string_thread.lock().unwrap();
            string.push_str("*");
        }
    });

    loop {
        thread::sleep(delay);
        let mut string = shared_string.lock().unwrap();
        string.push_str("*");
        println!("{}", string);
    }
}
