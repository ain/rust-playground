use std::sync::{Arc, Mutex};
use std::thread;

struct Bream {
    name: Arc<String>,
    speed: Arc<Mutex<String>>
}

impl Bream {
    fn new(name: Arc<String>, speed: Arc<Mutex<String>>) -> Bream {
        Bream { name: name, speed: speed }
    }

    fn swim(&self) {
        let mut speed = self.speed.lock().unwrap();
        speed.clear();
        speed.push_str("fast");

        println!("{} swims {}!", self.name, speed);
    }
}

pub fn refs_across_threads() {
    let name = Arc::new(String::from("Bream Flathead"));
    let speed = Arc::new(Mutex::new(String::from("slow")));
    println!("{} strong reference(s) for {}'s speed", Arc::strong_count(&speed), name);

    let bream = Bream::new(name.clone(), speed.clone());
    println!("{} supposed to swim {}", name, speed.lock().unwrap());

    let thread = thread::spawn(move || {
        bream.swim();
    });
    println!("{} strong reference(s) for {}'s speed", Arc::strong_count(&speed), name);

    thread.join().unwrap();
}
