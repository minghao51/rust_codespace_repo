use std::collections::HashMap;
fn main() {
    let mut count:HashMap<char,i32> = HashMap::new();
    count.insert('s',2);
    let val = count.get(&'s').unwrap(); //mut

    println!("test: {val}");
    let val = count.get(&'s').unwrap()+1; //mut
    println!("test: {val}");

    // # default dict
    // https://stackoverflow.com/questions/41417660/how-does-one-create-a-hashmap-with-a-default-value-in-rust
    let val = count.get(&'a').cloned().unwrap_or(0);
    println!("test: {val}");
    // println!("You guessed: {guess}");


    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

}
