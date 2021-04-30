fn main() {

    let result = sing (8, 5);
    println! ("{}", result);
}

pub fn sing(start: u32, end: u32) -> String {

    let mut all_part = String::new();

    for n in 1..5 {
        all_part.push_str("No more bottles of beer on the wall");

        println! ("{}", n);


    }


    all_part


}

