struct Ocean {
    name: String
}

struct Fish<'ttl> {
    name: String,
    ocean: &'ttl Ocean
}

pub fn ocean() {
    let atlantic = Ocean { name: String::from("Atlantic") };
    let cod = Fish { name: String::from("Cod"), ocean: &atlantic };

    println!("{} found in {} ocean",
        cod.name,
        cod.ocean.name); // Cod found in Atlantic ocean
}
