pub fn verse(n: u32) -> String {

    match n {
        0 => {
            let all_part =  String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
            all_part
        }
        1 => {
            let string_n : String = n.to_string();
            let first_part = String::from(" bottle of beer on the wall, ");
            let second_part = String::from(" bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
            let mut all_part = String::new();

            all_part += &string_n;
            all_part += &first_part;
            all_part += &string_n;
            all_part += &second_part;

            all_part
        }
        2 =>{
            let string_n : String = n.to_string();
            let first_part = String::from(" bottles of beer on the wall, ");
            let second_part = String::from(" bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
            let mut all_part = String::new();

            all_part += &string_n;
            all_part += &first_part;
            all_part += &string_n;
            all_part += &second_part;

            all_part

        }
        _ => {
            let number = n -1;

            let string_n : String = n.to_string();
            let string_number: String = number.to_string();
            let first_part = String::from(" bottles of beer on the wall, ");
            let second_part = String::from(" bottles of beer.\nTake one down and pass it around, ");
            let third_part = String::from(" bottles of beer on the wall.\n");
            let mut all_part = String::new();

            all_part += &string_n;
            all_part += &first_part;
            all_part += &string_n;
            all_part += &second_part;
            all_part += &string_number;
            all_part += &third_part;

            all_part
        }
    }


}

pub fn sing(start: u32, end: u32) -> String {

    (end..start+1).rev().map(|x| verse(x)).collect::<Vec<String>>().join("\n")

}

