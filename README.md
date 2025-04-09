## Program Structure and Concepts

- **Arrays** are used to store collections of related data:
    
    - `days` array stores ordinal names for each day of Christmas
    - `gifts` array stores the gifts in order from first to twelfth
- **Loops** are implemented to handle repetition:
    
    - **Outer loop** (`for days_index in 0..=11`) iterates through each day
    - **Inner loop** (`for gift_index in (0..=days_index).rev()`) handles gifts for each day
    - The `.rev()` method reverses the iteration order
- **Conditional logic** (`if gift_index == 0 && days_index > 0`) adds "And" before the partridge line except on the first day
    
- **String formatting** with `println!` displays the lyrics with proper structure
    

## Code Example

```rust
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
```

## Detailed Inner Loop Explanation

```rust
for gift_index in (0..=days_index).rev() {
    if gift_index == 0 && days_index > 0 {
        println!("And {}", gifts[0]);
    } else {
        println!("{}", gifts[gift_index]);
    }
}
```

- The inner loop uses `(0..=days_index).rev()` to:
    
    - Create a range from 0 up to the current day's index (e.g., 0 to 3 on the fourth day)
    - `.rev()` reverses this range so we count down (e.g., 3, 2, 1, 0 on the fourth day)
    - This gives us the correct gift ordering from newest to oldest
- The conditional statement handles the special "And" case:
    
    - `gift_index == 0` checks if we're at the partridge line (gifts array index 0)
    - `days_index > 0` checks if we're not on the first day
    - When both are true, we add "And" before the partridge line
    - On day 1, we print: "A partridge in a pear tree"
    - On day 2, we print: "Two turtle doves" and "And A partridge in a pear tree"
- Example iteration on the third day:
    
    1. `days_index = 2`, range is (0..=2) which is [0,1,2]
    2. After `.rev()`, iteration order is [2,1,0]
    3. First iteration: `gift_index = 2`, prints "Three French hens"
    4. Second iteration: `gift_index = 1`, prints "Two turtle doves"
    5. Third iteration: `gift_index = 0` and `days_index > 0`, prints "And A partridge in a pear tree"

## Program Flow Diagram

```
Initialize arrays (days, gifts)
↓
For each day (0 to 11):
    ↓
    Print day introduction
    ↓
    For each gift (from current day down to 0):
        ↓
        If (gift = partridge AND not first day):
            Print "And" + partridge line
        Else:
            Print gift line
```
