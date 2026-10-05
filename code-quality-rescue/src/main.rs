/*
    This is poorly written Python code with various issues.

    1. No meaningful identifiers, makes program much more harder to read and understand
    2. No comments or anything, it's virtually impossible for another programmer to understand what's going on especially due to the first issue above
    3. The prompts are also meaningless and the user wouldn't understand what they're required to input
    4. The lack of any whitespace can make this a bit harder on the eyes to look at

    ```python
        a=input("n")
        b=int(input("a"))
        c=float(input("p"))
        d=b*c
        print(a,b,c,d)
    ```

    Before we fix this code up, we need to understand what it's even doing to give the variables more meaningful identifiers and write good comments and such.

    Looking at the code, `a` is a string input with `b` being an integer and `c` being a float
    `d` is just the result of `b` and `c` being multiplied so it's most likely a float to because of `c`

    The final output is the string, the integer and float and the final calculated float `d`

    Now I can't justify my guess much but considering previous programs, I'm led to assume that this program inputs a name, price and quantity and calculates
    the total cost before outputting it all to the user. So that's what we'll be using to rewrite the program better.
*/

#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

fn main() {
    /*
        I added
            1. Meaningful identifiers
            2. Comments explaining what's going on
            3. Meaningful prompts so the user knows what to enter
            4. A bunch of spacing so the code is easier to read and go through especially thanks to the comments
    */

    // User name
    let mut name: String = String::new();
    prompt_str("Enter Name: ", &mut name);

    // Ask user for price and quantity of whatever
    let price: f32 = prompt_num("Enter Price: ");
    let quantity: u16 = prompt_num("Enter Quantity: ");

    // Output total cost
    let total_cost: f32 = price * quantity as f32;
    println!("{name}: {price} * {quantity} = {total_cost}");
}
