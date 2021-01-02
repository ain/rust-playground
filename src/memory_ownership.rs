pub fn primitive_ownership() {
    let vctr = vec!["this", "sits", "in", "the", "heap"];
    let vctr2 = vctr;
    //println!("{:?}", vctr); // fails compiler

    let ione = 1;
    let itwo = ione;
    println!("Primitive reference to 'ione' variable succeeds for following debug print: {:?}", ione);
}
