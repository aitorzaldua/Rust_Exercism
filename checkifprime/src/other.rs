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

