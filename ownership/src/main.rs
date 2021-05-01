fn main() {
    // let x = 5;
    // let y = x;
    // println!("The value of y: {}", y);
    // println!("The value of x: {}", x);

    // let a = String::from("x1");
    // let b = a; // move
    // println!("The value of a: {}", a); // borrow with moved value, panic!
    // println!("The value of b: {}", b);

    // let s1 = String::from("s1");
    // let s2 = s1; // moved
    // println!("Hi, {}", s1); // doesn't implement `Copy` trait, so panic!

    // let s1 = String::from("s1");
    // let s2 = s1.clone();
    // println!("Hi, {}", s1);
    // println!("Hi, {}", s2);

    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    //     println!("The value of r1: {}", r1);
    // }

    // let r2 = &mut s;
    // println!("The value of r2: {}", r2);

    // let mut s = String::from("hello");

    // let r1 = &s;

    // let r2 = &s;

    // let r3 = &mut s; // mutable / immutable borrow

    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);

    // let refer_to_nothing = dangle();

    // let s1 = String::from("a word");
    // let s2 = String::from("singleword");

    // println!("s1 first word: {}", first_word(&s1));
    // println!("s2 first word: {}", first_word(&s2));

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // first_word borrowed as immutable, now it borrowed as mutable!
    println!("The first word is: {}", word);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// &str or &String, both of them are ok, we prefer &str.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
