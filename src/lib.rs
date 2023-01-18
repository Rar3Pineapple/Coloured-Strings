/// This library takes in a str and returns a coloured String.


/// this function returns the String with the colour.
fn colourise(text: &str, colour: i8) -> String {
    return "\x1b[".to_owned() + &colour.to_string() + "m" + text + "\x1b[0m";
}


/// This function takes the &str, then calls the "colourise" function to convert it into a coloured string.
///
/// ___ 
///
/// Examples
/// ```
/// coloured("this is a black string", "black");
/// ```
/// 
pub fn colour(text: &str, colour: &str) -> String {
    match colour {
        ///normal colours
        "black" | "b" => return colourise(text, 30),
        "red" | "r" => return colourise(text, 31),
        "green" | "g" => return colourise(text, 32),
        "yellow" | "y" => return colourise(text, 33),
        "blue" | "bl" => return colourise(text, 34),
        "magenta" | "m" => return colourise(text, 35),
        "cyan" | "c" => return colourise(text, 36),
        "white"| "w" => return colourise(text, 37),
        
        /// bright colours
        "brightblack" | "bb" => return colourise(text, 90),
        "brightred" | "br" => return colourise(text, 91),
        "brightgreen" | "bg" => return colourise(text, 92),
        "brightyellow" | "by" => return colourise(text, 93),
        "brightblue" | "bbl"=> return colourise(text, 94),
        "brightmagenta" | "bm" => return colourise(text, 95),
        "brightcyan" | "bc" => return colourise(text, 96),
        "brightwhite" | "bw" => return colourise(text, 97),
        
        ///background colours
        "backgroundred" | "b-r" => return colourise(text, 41),
        "backgroundgreen" | "b-g" => return colourise(text, 42),
        "backgroundyellow" | "b-y" => return colourise(text, 43),
        "backgroundblue" | "b-b" => return colourise(text, 44),
        "backgroundmagenta" | "b-m" => return colourise(text, 45),
        "backgroundcyan" | "b-c" => return colourise(text, 46),
        "bacgroundwhite" | "b-w" => return colourise(text, 47),

        //bright background colours
        "backgroundbrightblack" | "b-bb" => return colourise(text, 100),
        "backgroundbrightred" | "b-br" => return colourise(text, 101),
        "backgroundbrightgreen" | "b-bg" => return colourise(text, 102),
        "backgroundbrightyellow" | "b-by" => return colourise(text, 103),
        "backgroundbrightblue" | "b-bbl" => return colourise(text, 104),
        "backgroundbrightmagenta" | "b-bm" => return colourise(text, 105),
        "backgroundbrightcyan" | "b-bc" => return colourise(text, 106),
        "backgroundbrightwhite" | "b-bw" => return colourise(text, 107),


        /// edge case
        _ => return colourise("no colour selected!", 93)


    }
}


