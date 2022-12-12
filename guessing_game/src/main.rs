use std::{io, cmp::Ordering}; //io -> mogli smo zanemariti (ali kasnije io::stdin() -> std::io::stdin()), cmp::Ordering -> enum koji koristimo dolje u match-u
use rand::Rng; //Rng je trait i on definira metode koje rand num gen implementira
fn main() {
    println!("Guess the number :)");
    let secret_number = rand::thread_rng().gen_range(1..=100); //thread_rng fija da nam random generator (seeded and local to current thread generator) i na njemu pozovemo fiju get_range(definiranu Rng traitom) koja kao argument prima range čije su gornje i donje granice otvorene
    let mut try_num: u32 = 1;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); //stvara novi string
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //read_line appenda sadržaj na  predani string (zato argument mora bit mut) & mora bit refrence jer se veže na objekt stringa a ne kopira
        
        //DEAD CODE
        //let guess: u8 = guess.trim().parse().expect("Please tipe a  number from 1 - 100."); //ovo se naziva shadowing (često se koristi kad želimo konvertat vrijednost iz 1 tipa u drugi (ne treba se nova jedinstvena varijabla)) (to smo trebali napraviti da bi match radio), trim metoda se riješi razmaka na početku i kraju i novog reda nastalog prilikom unosa teksta, parse metoda radi samo na znakovima koji se logički mogu konvertati u brojeve (podložna je greški pa vraća Result koji je dorbro provjeriti s expect)

        let guess: u8 = match guess.trim().parse() { //ovo je bolje od pretodne DEAD CODE linije jer pruža error handling a ne gasi samo program kad se radi o lošem inputu
            Ok(num) => num, //kad parse vrati dobar Result u koji upiše stvoreni broj
            Err(_) => continue, //kad parse ne vrati dobar Result ( _ znači da se uzimaju sve error vrijednosti bez obzira na njihovu vrijednost)
        };
        match guess.cmp(&secret_number) { //cmp je compare metoda (uspoređuje 2 ista tipa podataka)
            Ordering::Equal => { println!("You win."); break; },
            Ordering::Greater => println!("Too big."),
            Ordering::Less => println!("Too small.")
        }
        try_num += 1;
    }
    println!("Times you tried: {}", try_num);
    io::stdin().read_line(&mut String::new()).unwrap(); //ovo mogu koristiti na kraju programa da se terminal ne ugasi odmah, umjesto da u read_line ubacujem neku varijablu stvorit ću samo novi string te nakon što mi read_line baci Result, unwrapat ću ga bez error handlinga i istovremeno neću dobiti warning
}
