fn main() {

    let result = nth(5);

    println! ("result: {}", result);

}

pub fn nth(n: u32) -> u32 {

    let mut counter = 0;

    let prime_number = 0;


    for divider1 in 2..2147483647 {


        for divider2 in 2..divider1 {

            println! ("{}, {}, {}", divider1, divider2, counter);

            if divider1 % divider2 == 0 {
                break
            }
            else if divider2 == divider1 - 1 {
                counter += 1;
                if counter == n {
                    prime_number == divider1;
                    break
                }
            }

        }

    }

    prime_number
}

