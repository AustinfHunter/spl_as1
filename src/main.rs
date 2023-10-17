fn prog_one() {
    // TODO: Implement a program that utilizes at least 4 data types( example: int, float, string, boolean) 
    // and showcase 2 in built methods(per data type total =8) showcasing data manipulations (example :sum, average string replace)

}

fn prog_two() {
    // TODO: Include at least 2 major data structures (array, list, struct) 
    // and 2 major control structure( example: for loop, while loop, if-else, lambda functions etc)


}


//Program 3 demonstrates exception handling. Rust allows the use of the Result enum type to handle
//recoverable errors. In this example, the outer function takes a reference to a string literal as
//an argument and returns a string detailing the result of passing that reference to an inner
//function.
fn prog_three(s: &str) -> String {
    // TODO: Implement code that showcases exception handling or Concurrency.

    // Returns a result containing an error if the string contains anything other than uppercase or lowercase letters and containing nothing otherwise.
    // The result return type takes two generic parameters, T and E, where T is the type stored in Ok and E is the type stored in Err.
    fn alphabetical_or_err(input: &str) -> Result<(),String> {
        for b in input.chars() {
            if (b < 'A' || b > 'Z') && (b < 'a' || b > 'z') {
                return Err(format!("Not ASCII! Contains '{:}'", b));
            }
        }
        Ok(())
    }

    // The Result type has to be unwrapped in some way. One option is via pattern matching, that is
    // what is done here.
    match alphabetical_or_err(s) {
        Ok(()) => format!("No error for string: {:}", s),
        Err(e) => format!("Err: {:?}", e)
    }
}


fn main() {
    // Running and/or testing program one
    prog_one();
    // Running and/or testing program two
    prog_two();
    // Running and/or testing program three
    println!("{:}",prog_three("thisisok"));
    println!("{:}",prog_three("this is not ok"));
    println!("{:}",prog_three("thisisn0t0k"));
}
