fn main() {
    ownership_function();
    return_value_scope();
    references_borrowing();
    mutable_references();
    slices();
}

fn ownership_function() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("x is {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_value_scope() {
    let _s1 = gives_ownership();         

    let s2 = String::from("hello");     

    let _s3 = takes_and_gives_back(s2);  

} 

fn gives_ownership() -> String {             
                                             
    let some_string = String::from("hello"); 

    some_string                              
}

fn takes_and_gives_back(a_string: String) -> String { 

    a_string  

}

fn references_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slices() {
    //let s = String::from("Hello world!");

    let word = first_word("Hello world!");

    println!("First word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}