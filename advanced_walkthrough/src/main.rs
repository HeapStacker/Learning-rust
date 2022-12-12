fn main() {
    let s1 = String::from("hello"); //String je heap tip podatka
    let len = calculate_length(&s1); //ako string prenesemo bez &
    let mut some_string = String::from("Martin");
    change(&mut some_string);
    add_surname(&mut some_string);
    println!("{some_string}");
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;      //ova linija koda ne bude radila jer u 1 trenutku može postojati samo 1 &mut (preventira data race)
    //println!("{}, {}", r1, r2);
    println!("{}", r1);
    let r1 = &s; // no problem
    let r2 = &s; // no problem (ako necemo mijenjati refrence ako je imutable možemo ih imati više)
    //let r3 = &mut s; // BIG PROBLEM (ne može se u isto vrijeme posuditi kao mutable i imutable i ne možemo tip koji je imutable posuditi kao mutable)
    //println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, and {}", r1, r2);
    //let reference_to_nothing = dangle(); //rust preventira stvaranje dangling refrence-a
    //RECAP -> možemo u nekom trenutku imati 1 mut ref ili više imut. revrenca
    let string = String::from("Martin Tomaskovic");
    let str = first_word(&string);
    println!("{str}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",  world");
}

fn add_surname(some_string: &mut String) {
    some_string.push_str(", Tomaskovic");
}

//ova funkcija ne radi (zamijenili smo ju s prethodnom) jer je refrence do defaltu imutable tip te zato s &Strgin ne možemo promijeniti string u ovoj fiji
// fn change(some_string: &String) {
//     some_string.push_str(" ,world");
// }

//ne možemo stvoriti "dangling refrence" (npr. refrence koji pokazuje na područje u mem koje se obrisalo) jer rust baca error 
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i];
        } 
    }
    &s[..]
}