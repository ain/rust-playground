use std::rc::Rc;
use std::cell::RefCell;

struct Sea {
    name: String,
    fish: Vec<Rc<RefCell<Fish>>>
}

impl Sea {
    fn new(name: &str) -> Sea {
        Sea {
            name: name.into(),
            fish: Vec::new()
        }
    }

    fn add_fish(fish: Rc<RefCell<Fish>>, sea: Rc<RefCell<Sea>>) {
        fish.borrow_mut().seas.push(sea.clone());
        sea.borrow_mut().fish.push(fish);
    }
}

struct Fish {
    name: String,
    seas: Vec<Rc<RefCell<Sea>>>
}

impl Fish {
    fn new(name: &str) -> Fish {
        Fish {
            name: name.into(),
            seas: Vec::new()
        }
    }
}

pub fn circ_refs() {
    let herring = Rc::new(RefCell::new(Fish::new("Hopeful Herring")));
    let baltic_sea = Rc::new(RefCell::new(Sea::new("Baltic Sea")));

    Sea::add_fish(herring, baltic_sea.clone());

    let sprat = Rc::new(RefCell::new(Fish::new("Sporty Sprat")));
    Sea::add_fish(sprat, baltic_sea);
}
