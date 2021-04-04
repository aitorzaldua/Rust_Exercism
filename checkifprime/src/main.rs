fn main() {

    let if_prime = 15;

    for divider1 in 2..if_prime {

        let mut counter = 0;

        let resto = if_prime % divider1;

        println! ("{} % {} = {}",if_prime, divider1, resto);

        match resto {
            0 => counter +=1,
            _ => println!("primo"),
        }

    }

}
