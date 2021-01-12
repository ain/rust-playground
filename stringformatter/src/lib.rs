pub fn formatted_string() {
    let formatted = format!(
        "{0} {1} was released {day}. {1} is the latest {0} version.",
        "Node.js",
        15.5,
        //date = "22.12.2020", // fails compilation
        day = "today"
    );

    println!("{}", formatted); // Node.js 15.5 was released today. 15.5 is the latest Node.js version.
}
