trait Fish {
    fn create<S: Into<String>>(name: S, predator: bool) -> Self where Self: Sized;

    fn name(&self) -> &String;

    fn predator(&self) -> bool;

    fn eat(&self) {
        println!("{} had a {} meal",
            self.name(),
            if self.predator() { "pescetarian" } else { "vegan" });
    }
}

struct Herring {
    name: String,
    predator: bool,
    location: &'static str
}

struct Salmon {
    name: String,
    predator: bool
}

impl Fish for Herring {
    fn create<S: Into<String>>(name: S, predator: bool) -> Herring {
        Herring{ name: name.into(), predator: predator, location: "North Atlantic" }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn predator(&self) -> bool {
        self.predator
    }
}

impl Drop for Herring {
    fn drop(&mut self) {
        println!("{} died", self.name());
    }
}

impl Fish for Salmon {
    fn create<S: Into<String>>(name: S, predator: bool) -> Salmon {
        Salmon{ name: name.into(), predator: predator }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn predator(&self) -> bool {
        self.predator
    }
}

pub fn fishtank() {
    let herrings_name: &'static str = "Humble Herring";
    let herring: Herring = Fish::create(herrings_name, false);
    herring.eat(); // Humble Herring had a vegan meal
    println!("{} was in {}", herring.name(), herring.location); // Humble Herring was in North Atlantic

    let salmons_name: String = String::from("Shallow Salmon");
    let salmon: Salmon = Fish::create(salmons_name, true);
    salmon.eat(); // Shallow Salmon had a pescetarian meal

    println!("Fishtank lives");

    let mut fishtank_register: Vec<Box<dyn Fish>> = Vec::new();
    fishtank_register.push(Box::new(herring));
    fishtank_register.push(Box::new(salmon));
    for fish in fishtank_register {
        println!("{} found in register", fish.name());
    }
}
