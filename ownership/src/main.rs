fn main() {
    // --- Ownership rules ---
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point onward
        // do stuff with s
    }// this scope is now over, s is no longer valid

    let x = 5;
    let y = x; //copy

    let s1 = String::from("hello");
    //let s2 = s1; // Move (not shallow copy)
    let s2 = s1.clone();

    println!("{} world!", s1);

    /*let ss = String::from("test");
    takes_ownership(ss);
    println!("{}", ss); // doesn't work as ownership has been moved to the function
    */

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    let s1 = String::from("hello");
    let len = calculate_length(&s1); //tuple
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; NOT POSSIBLE as you cant have immutable and mutable refences at the same time 
    println!("{}, {}", r1, r2);

    let r3 = &mut s; //POSSible because the scope of r1 and r2 have ended here, so it's mutable again
    change(r3);
    println!("{}", r3);

    let r1 = &s; //POSSIBLE because scope of r3 has ended
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let mut s = String::from("hello world!");
    let hello = &s[..5];
    let world = &s[6..];
    let s2 = "hello world";

    let word = first_word(&s);

}

fn takes_ownership(some_string: String)
{
    println!("{}", some_string);
}

fn gives_ownership() -> String
{
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String
{
    a_string
}

fn makes_copy(some_integer: i32)
{
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize //borrowing
{
    //references are immutable by default
    let length = s.len();
    length
}

//mutable reference, argument must be mutable, input also
fn change(some_string: &mut String)
{
    some_string.push_str(" world!");
}

fn first_word(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    &s[..]
}