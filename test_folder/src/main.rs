fn main() {

    let n = 5;
    let mut counter = 0;
    let mut prime = 0;

    for divider1 in 2..2147483647 {


        for divider2 in 2..divider1{

            let resto = divider1 % divider2;

            println! ("{} % {} = {}", divider1, divider2, resto);

            if resto == 0 {
                break
            }
            else {

                if divider2 == divider1 - 1 {
                    counter += 1;
                }
            }

            println! ("counter: {}", counter);

        }
        if counter == n {
            prime = divider1;
            println! ("fin de ciclo");
            break
        }

    }


    println! ("fuera del ciclo for, {}", prime);

}

