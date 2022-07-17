fn main() {
    let mut christimas_song = String::new();

    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];

    const VERSES: [&str; 12] = [
        "A partridge in a pear tree\n",
        "Two turtledoves\n",
        "Three French hens\n",
        "Four calling birds\n",
        "Five gold rings (five golden rings)\n",
        "Six geese a-laying\n",
        "Seven swans a-swimming\n",
        "Eight maids a-milking\n",
        "Nine ladies dancing\n",
        "Ten lords a-leaping\n",
        "Eleven pipers piping\n",
        "Twelve drummers drumming\n",
    ];

    let mut previous_verse = String::new();

    for n in 0..11 {
        let day = DAYS[n];
        let verse = VERSES[n];

        let final_verse = if n > 0 {
            "And a partridge in a pear tree\n\n"
        } else {
            "\n"
        };

        let base = "On the ".to_owned()
            + &day.to_owned()
            + &" day of Christmas, my true love sent to me\n".to_owned()
            + &verse.to_owned()
            + &previous_verse.to_owned()
            + &final_verse;

        if n > 0 {
            previous_verse = verse.to_owned() + &previous_verse;
        }

        christimas_song = christimas_song + &base;
    }

    println!("{christimas_song}");
}

/*
 "String and &str are different beasts.
 String can live and be used on its own, but &str is just a reference to part of String.
 So, &str can live as long as referenced String lives."

 https://stackoverflow.com/questions/54056268/temporary-value-is-freed-at-the-end-of-this-statement
*/
