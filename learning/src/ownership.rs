// Ownership is a concept in rust that is used to manage memory

// three ownership rules are
// 1. Each value in rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


// Rule 1:  Each value in rust has a variable that is called its owner.
fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}