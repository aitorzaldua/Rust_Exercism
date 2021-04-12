fn main() {

    for number in (1..100).rev() {

        let number2 = number -1;

        println!("{} bottles of beer on the wall, {} bottles of beer.", number, number);

        if number2 > 0 {
            println!("Take one down and pass it around, {} bottles of beer on the wall.", number2);
        }
        else {
            println!("Take one down and pass it around, no more bottles of beer on the wall.");
        }

    }

    println! ("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.");

}

