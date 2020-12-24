use std::collections::HashSet;

pub fn hashset_subset() {
    let mut ee_colours = HashSet::new();
    ee_colours.insert("Blue");
    ee_colours.insert("Black");
    ee_colours.insert("White");
    println!("{:?}", ee_colours); // {"Black", "White", "Blue"}

    ee_colours.insert("Blue");
    println!("{:?}", ee_colours); // {"Black", "White", "Blue"}

    let mut fi_colours = HashSet::new();
    fi_colours.insert("Blue");
    fi_colours.insert("White");
    println!("is {:?} a subset of {:?}: {}", fi_colours, ee_colours,
        fi_colours.is_subset(&ee_colours)); // is {"White", "Blue"} a subset of {"Blue", "White", "Black"}: true
}
