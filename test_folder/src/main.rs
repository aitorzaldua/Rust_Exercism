fn main() {


    let number = 13;
    let  mut counter = 1;


    for divider1 in 2..number {


        for divider2 in 2..divider1{

            let resto = divider1 % divider2;

            
            if resto == 0 {
                break
            }
            else {
                println! ("{} % {} = {}", divider1, divider2, resto);
                if divider2 == divider1 + 1 {
                    counter += 1
                }
            }

        }

    }
    println! ("numeros primos hasta el number: {}", counter);


}
