use std::rc::Rc;

struct Bream {
    name: Rc<String>
}

impl Bream {
    fn new(name: Rc<String>) -> Bream {
        Bream { name: name }
    }

    fn swim(&self) {
        println!("{} swims!", self.name);
    }
}

pub fn strong_refs() {
    let name = Rc::new(String::from("Bream Flathead"));
    println!("{} strong reference(s) for 'name' prior construct scope", Rc::strong_count(&name));
    {
        let bream = Bream::new(name.clone());
        bream.swim();
        println!("{} strong reference(s) for 'name' in construct scope", Rc::strong_count(&name));
    }
    println!("{} strong reference(s) for 'name' post construct scope", Rc::strong_count(&name));
}
