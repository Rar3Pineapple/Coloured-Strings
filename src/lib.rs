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
/// ```rust
/// colour("this is a black string", "black");
/// colour("this is a underlined string", "u");
/// ```
/// ___
/// Available Colours
/// ```rust 
///    // normal colours
///     "black" | "b"
///     "red" | "r"
///     "green" | "g"
///     "yellow" | "y"
///     "blue" | "bl"
///     "magenta" | "m"
///     "cyan" | "c"
///     "white"| "w"
///     
///     // bright colours
///     "brightblack" | "bb" 
///     "brightred" | "br"   
///     "brightgreen" | "bg"   
///     "brightyellow" | "by"  
///     "brightblue" | "bbl" 
///     "brightmagenta" | "bm"  
///     "brightcyan" | "bc"  
///     "brightwhite" | "bw"  
///     
///     //background colours
///     "backgroundred" | "b-r"   
///     "backgroundgreen" | "b-g"  
///     "backgroundyellow" | "b-y"  
///     "backgroundblue" | "b-b"  
///     "backgroundmagenta" | "b-m"  
///     "backgroundcyan" | "b-c"  
///     "bacgroundwhite" | "b-w" 
///        
///     //bright background colours
///     "backgroundbrightblack" | "b-bb" 
///     "backgroundbrightred" | "b-br" 
///     "backgroundbrightgreen" | "b-bg" 
///     "backgroundbrightyellow" | "b-by"  
///     "backgroundbrightblue" | "b-bbl"  
///     "backgroundbrightmagenta" | "b-bm"  
///     "backgroundbrightcyan" | "b-bc"  
///     "backgroundbrightwhite" | "b-bw"  
///
///    // other styles
///     "clear" | "cl"  
///     "dimmed" | "d"   
///     "italic" | "i"   
///     "underline" | "u"  
///     "blink" | "bli"   
///     "reversed" | "re"   
///     "hidden" | "h"  
///     "strikethrough" | "s"  
/// ```
/// 
pub fn colour(text: &str, colour: &str) -> String {
    match colour {
        //normal colours
        "black" | "b" => return colourise(text, 30),
        "red" | "r" => return colourise(text, 31),
        "green" | "g" => return colourise(text, 32),
        "yellow" | "y" => return colourise(text, 33),
        "blue" | "bl" => return colourise(text, 34),
        "magenta" | "m" => return colourise(text, 35),
        "cyan" | "c" => return colourise(text, 36),
        "white"| "w" => return colourise(text, 37),
        
        // bright colours
        "brightblack" | "bb" => return colourise(text, 90),
        "brightred" | "br" => return colourise(text, 91),
        "brightgreen" | "bg" => return colourise(text, 92),
        "brightyellow" | "by" => return colourise(text, 93),
        "brightblue" | "bbl"=> return colourise(text, 94),
        "brightmagenta" | "bm" => return colourise(text, 95),
        "brightcyan" | "bc" => return colourise(text, 96),
        "brightwhite" | "bw" => return colourise(text, 97),
        
        //background colours
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

        // other styles
        "clear" | "cl" => return colourise(text, 1),
        "dimmed" | "d"  => return colourise(text, 2),
        "italic" | "i" => return colourise(text, 3),
        "underline" | "u" => return colourise(text, 4),
        "blink" | "bli" => return colourise(text, 5),
        "reversed" | "re"  => return colourise(text, 7),
        "hidden" | "h" => return colourise(text, 8),
        "strikethrough" | "s" => return colourise(text, 9),
        // edge case
        _ => return colourise("no colour selected!", 93)


    }
}

/// This function takes a &str, which acts as the text; a boolean value to determine if the colour should be applied to the foreground or background; and red, blue and green values to determine colour.
///
/// ---
/// Examples
///
/// ```rust
/// custom_colour("foreground", true, 200, 240, 100);
/// custom_colour("background", false, 200, 100, 200);
/// ```
///
/// ---
pub fn custom_colour(text: &str, foreground: bool, red: u8, green: u8, blue: u8) -> String {
    // turn red, blue and green into strs to prepare for formatting
    let r = red.to_string(); let g = green.to_string(); let b = blue.to_string();
    // match statement to determine if foreground or background is being called
    match foreground {
        // foreground
        true => return "\x1b[".to_owned() + &format!("38;2;{};{};{}", r, g, b).to_string() + "m" + text + "\x1b[0m",
        // background
        false => return "\x1b[".to_owned() + &format!("48;2;{};{};{}", r, g, b).to_string() + "m" + text + "\x1b[0m",

    }
}
