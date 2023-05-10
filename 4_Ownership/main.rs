fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    ownership_functions();
    references();
    mutable_references();
    dangling_references();
}

fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
}

fn dangling_references() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn ownership_functions(){
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{s}");
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn slice_type() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        } 
    }
    &s[..]

    s.len()
}