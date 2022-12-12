fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number != 0 {
        println!("Number was > 0");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut i: u8 = 0;

    loop {
        i+=1;
        if i % 3 == 0 { continue; }
        println!("{}", i);
        if i >= 10 { break; }
    }

    println!("");
    println!("");
    for number in (1..5).rev() { //odbrojava unazad (ne možemo napisati (5..1) )
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    let mut s = "dsajodijasd";
    println!("{s}");
    s = "oasidjajosd";
    println!("{s}");

    ownership();
}



fn ownership() {
    let mut str = String::from("hello"); //ovako dobimo string tip podatka spremljen na heapu i kojeg mogu mijenjati
    str.push_str(" asdfjasdfasdf");
    println!("{}", str);

    let str2 = str; //nije kopiranje nego micanje, move (memory safety) - nismo kopirali 1. string u 2. već smo napravili da pointer 2. stringa pokazuje na adresu prvog te radi memorijske sigurnosti str više nije validan (to je napravljeno tako da ne možemo 2 puta osloboditi isto mjesto u memoriji (kad objekt izađe iz scope-a))
    
    //println!("{str}"); dakle ova linija koda ne radi (razlog je naveden u prošlom komentaru)

    println!("{str2}"); //mogu printat str2 al str1 ne mogu


    let s1 = String::from("orefj");
    let s2 = s1.clone(); //ovako se kopira (nije move) s clone se radi deep copy (expensive code)

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x; //pošto su oba na stacku ne moramo paziti na kopiranje (da ćemo imati 2. vrijednosti koje pokazuju na istu adresu) ovaj expression smo mogli napisati kao  ... = x.clone(); tu su shallow copy i deep copy isti
    println!("x = {}, y = {}", x, y);


    let s = String::from("hello");  // s uđe u scope

    takes_ownership(s); // vrijednost s-a uđe u funkciju...
                                    // ... s ne vrijedi više ovdje

    let num = 5;                 // x uđe u scope

    makes_copy(num);     // num would move into the function,
                                      // but i32 is Copy, so it's okay to still
                                      // use num afterward


    giving_ownership();
    
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.









fn giving_ownership() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("{s1} {s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}