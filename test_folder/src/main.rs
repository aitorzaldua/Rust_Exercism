fn main() {

    let result = sing (8, 5);
    println! ("{}", result);
}

pub fn sing(start: u32, end: u32) -> String {

    match start {
        0 => {
            let all_part =  String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
            all_part
        }
        1 => {
            let string_start : String = start.to_string();
            let first_part = String::from(" bottle of beer on the wall, ");
            let second_part = String::from(" bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
            let mut all_part = String::new();

            all_part += &string_start;
            all_part += &first_part;
            all_part += &string_start;
            all_part += &second_part;

            all_part
        }
        2 =>{
            let string_start : String = start.to_string();
            let first_part = String::from(" bottles of beer on the wall, ");
            let second_part = String::from(" bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
            let mut all_part = String::new();

            all_part += &string_start;
            all_part += &first_part;
            all_part += &string_start;
            all_part += &second_part;

            all_part

        }
        _ => {
            if start > end {


                let number = start -1;

                let string_start : String = start.to_string();
                let string_number: String = number.to_string();
                let first_part = String::from(" bottles of beer on the wall, ");
                let second_part = String::from(" bottles of beer.\nTake one down and pass it around, ");
                let third_part = String::from(" bottles of beer on the wall.\n");
                let mut all_part = String::new();

                all_part += &string_start;
                all_part += &first_part;
                all_part += &string_start;
                all_part += &second_part;
                all_part += &string_number;
                all_part += &third_part;

                all_part

            }

            else {
                let all_part =  String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
                all_part
            }
        }

    }




}