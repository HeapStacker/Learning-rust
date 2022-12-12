const THIS_IS_CONST: u32 = 129380123; //const tip je uvijek imutable (ne mogže biti mut) i njegov tip mora obavezno biti naveden (mogu biti deklarirane u bilo kojem scope-u (i globalnom))

fn data_types() {
    //integer data types...
    //8-bit	    i8	u8
    //16-bit	i16	u16
    //32-bit	i32	u32
    //64-bit	i64	u64
    //128-bit	i128	u128
    //arch	    isize	usize (ovisi o arhitekturi tvog proc (znači 64 ili 32 bitni broj))

    //integer literal forms... (brojevnim literali se može prikazati tip broj npr. 57u8)
    //Number literals	Example
    //Decimal	        98_222
    //Hex	            0xff
    //Octal	            0o77
    //Binary	        0b1111_0000
    //Byte (u8 only)	b'A'

    //numeric operations...
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;

    //bool type...
    let t = true;
    let f: bool = false; // with explicit type annotation

    //char type...
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //compounds...
    //tuple
    let tup = (500, 6.4, 1); //ili let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //println!("The value of y is: {y}");
    println!("We can access tuple elements like this too: {}, {}, {}", tup.0, tup.1, tup.2);

    //array... (statički alocirani moraš im znati veličinu, za razliku od tupla svi elementi su istog tipa)
    let a = [1, 2, 3, 4, 5]; //možeš napisati i let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // for int in a {
    //     println!("{int}");
    // }
    //ili...
    for i in 0..a.len() - 1 {
        println!("{}", a[i]);
    }

    let b = [3; 5]; //možeš i ovak napisat array (da svi elementi imaju jednaku vrijednost...)
    for i in b {
        println!("{i}");
    }

    println!("{} {} {} {} {} {} : {} {} : {c} {z} {heart_eyed_cat}", sum, difference, product, quotient, floored, remainder, t, f);
}

fn main() {
    println!("Printam konstantu samo da nebi bilo warninga {}", THIS_IS_CONST);

    let mut x = 5; //mora biti mut kako bi se mogao promijeniti kasnije
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let y = 5;
    let y = y + 1; //ovo je zasjenjenje
    {
        let y = y * 2; //ovo je isto zasjenjenje koje prestanedjelovati nakon što napusti ovaj scope
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "   ";
    //spaces = spaces.len(); ne može se zamijeniti tip &str s brojem
    let spaces = spaces.len(); //ali možemo sa zasjenjenjem
    println!("{spaces}");

    data_types();
}
