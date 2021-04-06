use std::io;


fn main() {

    println! ("Este programa adivina si un número es primo o no");
    println! ("!Además, te va a ofrecer un montón de datos interesantes!");
    println! ("Comencemos, inserta un número entre 1 y... mmm ¡4000000000!");

    let mut if_prime = String::new();
    let mut counter_no = 0;
    //let mut counter_yes = 0;

    io::stdin()
            .read_line(&mut if_prime)
            .expect("Failed to read line");

    let if_prime: u32 = if_prime.trim().parse().expect("Please type a number!");


    for divider1 in 2..if_prime {

        let resto = if_prime % divider1;

        //println! ("{} % {} = {}",if_prime, divider1, resto);

        match resto {
            0 => counter_no +=1,
            _ => continue,
        }

    }

    if counter_no != 0 {
        println! ("El número {} NO es un número primo", if_prime);
    }
    else {
        println! ("El número {} SI es un número primo", if_prime);
    }

    //println!("counter_no = {}, counter_yes = {}", counter_no, counter_yes);

}
