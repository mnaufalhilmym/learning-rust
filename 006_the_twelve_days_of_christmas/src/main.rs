const GIFT_LIST: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "french hens",
    "calling birds",
    "gold rings",
    "geese a laying",
    "swans a swimming",
    "maids a milking",
    "ladies dancing",
    "lords of leaping",
    "pipers piping",
    "drummers drumming",
];

fn main() {
    for day_num in 0..GIFT_LIST.len() {
        let day = match day_num {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            11 => "twelfth",
            _ => "",
        };
        let mut verse = format!("On the {day}");
        verse += " day of Christmas\nmy true love sent to me:\n";
        for gift_num in (0..day_num + 1).rev() {
            if gift_num == 0 {
                if day_num > 1 {
                    verse += "and "
                }

                verse += &format!("a {}.\n", GIFT_LIST[gift_num]);
            } else {
                verse += &format!("{} {},\n", gift_num + 1, GIFT_LIST[gift_num])
            }
        }
        println!("{}", verse)
    }
}
