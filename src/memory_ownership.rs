pub fn primitive_ownership() {
    let vctr = vec!["this", "sits", "in", "the", "heap"];
    let vctr2 = vctr;
    //println!("{:?}", vctr); // fails compiler

    let ione = 1;
    let itwo = ione;
    println!("Primitive reference to 'ione' variable succeeds for following debug print: {:?}", ione);

    let mut vec2 = vec!["this", "can", "be", "iterated", "but", "not", "pushed"];
    for i in vec2 {
        println!("i in vec2 is {}", i);
        //&vec2.push("sure?"); // fails compiler
    }
}
