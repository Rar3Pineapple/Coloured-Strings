# coloured-strings
[![banner](https://i.postimg.cc/vTTkDL5Y/Screenshot-2023-01-22-at-20-54-16.png)](https://postimg.cc/LgdNbPyw)
Rust crate that allows coloured strings to be printed to the terminal with the smallest file size neccessary.
NOTE: This version supports true colour in exchange for a slightly larger file size. If rgb colour is not neccessary, please download the previous verion instead.
```rust
  colourise("this is a green string", "green");
  colourise("this is a red string", "r");
  colourise("this is a string with a strikethrough", "s");
  colourise(&colourise("this string has white text and a blue background", w")[..], "b-bl"));
  custom_colour("A custom string!", true, 255, 100, 200);
```
## How to Use
Run ``` cargo add coloured-strings```
Afterwards, write this in your file ```use coloured_strings```

Example:
```rust
use coloured_strings::*;

fn main() {
    println!("{}", colour("I love coloured text!", "brightgreen");
}
```


## Current Colours
 ```rust
    // normal colours
     "black" | "b"
     "red" | "r"
     "green" | "g"
     "yellow" | "y"
     "blue" | "bl"
     "magenta" | "m"
     "cyan" | "c"
     "white"| "w"

     // bright colours
     "brightblack" | "bb"
     "brightred" | "br"
     "brightgreen" | "bg"
     "brightyellow" | "by"
     "brightblue" | "bbl"
     "brightmagenta" | "bm"
     "brightcyan" | "bc"
     "brightwhite" | "bw"

     //background colours
     "backgroundred" | "b-r"
     "backgroundgreen" | "b-g"
     "backgroundyellow" | "b-y"
     "backgroundblue" | "b-b"
     "backgroundmagenta" | "b-m"
     "backgroundcyan" | "b-c"
     "bacgroundwhite" | "b-w"

     //bright background colours
     "backgroundbrightblack" | "b-bb"
     "backgroundbrightred" | "b-br"
     "backgroundbrightgreen" | "b-bg"
     "backgroundbrightyellow" | "b-by"
     "backgroundbrightblue" | "b-bbl"
     "backgroundbrightmagenta" | "b-bm"
     "backgroundbrightcyan" | "b-bc"
     "backgroundbrightwhite" | "b-bw"

    // other styles
     "clear" | "cl"
     "dimmed" | "d"
     "italic" | "i"  
     "underline" | "u"
     "blink" | "bli"  
     "reversed" | "re"
     "hidden" | "h"
     "strikethrough" | "s"

```

