// out of bounds conditions
//fn main() {
//    let v = vec![1, 2, 3];
//
//    v[99];
//}

// main with a different return type
//use std::error::Error;
//use std::fs::File;
//
//fn main() -> Result<(), Box<dyn Error>> {
//    let f = File::open("hello.txt")?;
//
//    Ok(())
//}

// lifetimes experiments
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// the following function signature fails to compile (regardless of what's inside the function body): fn longest(x: &str, y: &str) -> &str {
// associating a lifetime reference in front of all parameters, plus the return type, plus the function name itself
fn longest<'_a_>(x: &'_a_ str, y: &'_a_ str) -> &'_a_ str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

