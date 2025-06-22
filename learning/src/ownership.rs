// Ownership is a concept in rust that is used to manage memory

// three ownership rules are
// 1. Each value in rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


fn main(){
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = String::from("hello world");
    println!("s2: {}", s2);  // Rule 2:  There can only be one owner at a time. it will throw an error if s1 is printed as the ownership of s1 is transferred to s2
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2, len); // Rule 1:  Each value in rust has a variable that is called its owner.

    let len1 = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len1);
    printloss(&s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn printloss(s: &String) {
    println!("s: {}", &s);
}

