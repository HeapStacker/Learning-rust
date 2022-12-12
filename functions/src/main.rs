fn expression_and_statement() {
    //ova cijela stvar je statement a stvar u {} zagradama je expression
    let y = {
        let x = 3;
        x + 1
    };
    //let y = (let x = 1);  poanta je da ne možeš statementu pridijeliti vrijednost statementa
    println!("{y}");
}

fn five() -> u8 { //funkcija koja vraća vrijednost
    5
}

fn main() {
    println!("{}", five());
    expression_and_statement();
    print_labeled_measurement(5, 'h');}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
