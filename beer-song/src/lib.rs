pub fn verse(n: u32) -> String {


    if n == 0 {

        let code = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        code
    }

    else {

        //let number = n -1;

        let s: String = n.to_string();

        //let code = String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");

        s

    }


}

pub fn sing(start: u32, end: u32) -> String {

    let code = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");

    code

}

