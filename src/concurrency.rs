use std::thread;
use std::time;

pub fn spawner() {

    println!("Spotted Fish in the sea!");

    let salmon = thread::spawn(|| {
        for _ in 1..5 {
            print!(" Salmon");
            thread::sleep(time::Duration::from_millis(50));
        }
    });

    let pike = thread::spawn(|| {
        for _ in 1..5 {
            print!(" Pike");
            thread::sleep(time::Duration::from_millis(50));
        }
    });

    for _ in 1..5 {
        print!(" Cod");
        thread::sleep(time::Duration::from_millis(50));
    }

    salmon.join();
    pike.join();
}
