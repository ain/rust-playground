struct Sea {
    name: String
}

impl Sea {
    fn fish(&self, ecosystem: Ecosystem) -> Vec<String> {
        ecosystem.habitats.iter()
            .filter(|&h| h.sea.name == self.name)
            .map(|h| h.fish.name.clone())
            .collect()
    }
}

struct Fish {
    name: String
}

struct Habitat<'a> {
    fish: &'a Fish,
    sea: &'a Sea
}

impl<'a> Habitat<'a> {
    fn new(fish: &'a Fish, sea: &'a Sea) -> Habitat<'a> {
        Habitat { fish, sea }
    }
}

struct Ecosystem<'a> {
    habitats: Vec<Habitat<'a>>
}

impl<'a> Ecosystem<'a> {
    fn new() -> Ecosystem<'a> {
        Ecosystem { habitats: Vec::new() }
    }

    fn inhabit(&mut self, fish: &'a Fish, sea: &'a Sea) {
        self.habitats.push(Habitat::new(fish, sea));
    }
}

pub fn circ_refs() {
    let herring = Fish { name: "Humble Herring".into() };
    let sea = Sea { name: "Baltic Sea".into() };

    let mut ecosystem = Ecosystem::new();
    ecosystem.inhabit(&herring, &sea);

    for f in sea.fish(ecosystem) {
        println!("Baltic Sea inhabited by {}", f);
    }
}
