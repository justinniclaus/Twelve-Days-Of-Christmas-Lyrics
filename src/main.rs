fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    
    for days_index in 0..=11 {
        println!("\nOn the {} day of Christmas, my true love sent to me:", days[days_index]);
        
        for gift_index in (0..=days_index).rev() {
            if gift_index == 0 && days_index > 0 {
                println!("And {}", gifts[0]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }
    }
}


