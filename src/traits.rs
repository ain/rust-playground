trait Fish {
    fn create(name: &'static str, predator: bool) -> Self;

    fn name(&self) -> &'static str;

    fn predator(&self) -> bool;

    fn eat(&self) {
        println!("{} had a {} meal",
            self.name(),
            if self.predator() { "pescetarian" } else { "vegan" });
    }
}

struct Herring {
    name: &'static str,
    predator: bool
}

struct Salmon {
    name: &'static str,
    predator: bool
}

impl Fish for Herring {
    fn create(name: &'static str, predator: bool) -> Herring {
        Herring{ name: name, predator: predator }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn predator(&self) -> bool {
        self.predator
    }
}

impl Fish for Salmon {
    fn create(name: &'static str, predator: bool) -> Salmon {
        Salmon{ name: name, predator: predator }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn predator(&self) -> bool {
        self.predator
    }
}

pub fn fishtank() {
    let herring:Herring = Fish::create("Humble Herring", false);
    herring.eat(); // Humble Herring had a vegan meal

    let salmon:Salmon = Fish::create("Shallow Salmon", true);
    salmon.eat(); // Shallow Salmon had a pescetarian meal
}
