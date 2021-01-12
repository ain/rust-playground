extern crate rand;

use rand::Rng;

pub fn random_boolean() {
    let mut rng = rand::thread_rng();

    for _ in 1..10 {
        let boolean:bool = rng.gen();
        println!("Random boolean: {:?}", boolean)
    }
}

