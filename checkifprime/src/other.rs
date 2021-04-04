pub fn nth(n:u32) -> u32 {

    let mut counter = 0;

    for divider1 in 2..n + 1 {



        for divider2 in 2..divider1 {

            let resto = divider1 % divider2;

            if resto == 0 {
                break
            }
            else {

                if divider2 == divider1 - 1 {
                    counter += 1;
                }
            }

        }

    }
    counter
}

fn main() {

    let if_prime = 12;

    for divider1 in 2..if_prime + 1 {

        let resto = if_prime % divider1;

        if resto == 0 {
            println! ("no es primo");
        }
        else {
            println! ("no es primo");
        }
    }

}