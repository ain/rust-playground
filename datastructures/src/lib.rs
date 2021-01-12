pub fn typed_matrix() {
    let mtx: [[i32;3];2] = [
        [123, 3, 43],
        [2, 32, 102]
    ];
    println!("mtx is {:?}", mtx); // prints [[123, 3, 43], [2, 32, 102]]
}

pub fn tuple_destruct() {
    let tpl = (234, 2.3454);
    let (int, float) = tpl;
    println!("tuple itself is {:?}", tpl); // tuple itself is (234, 2.3454)
    println!("tuple's elements are {}, {}", tpl.0, tpl.1); // tuple's elements are 234, 2.3454
    println!("tuple's int is {} and float is {}", int, float); // tuple's int is 234 and float is 2.3454
}
