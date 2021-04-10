pub fn nth(n: u32) -> u32 {


    let mut counter = 0;
    let mut prime = 0;

    for divider1 in 2..2147483647 {


        for divider2 in 2..divider1{

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
        if counter == n {
            prime = divider1;
            println! ("fin de ciclo");
            break
        }

    }


    prime

}
