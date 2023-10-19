    use std::collections::LinkedList;


    // Program 1 demonstrates the use of 4 data types
    // with 2 methods of data manipulations
    fn prog_one() {

            // Integer Data Type
            let integer1: i32 = 5;
            let integer2: i32 = 10;        
            // Floating-point Data Type
            let float1: f64 = 3.5;
            let float2: f64 = 2.0;
            // String Data Type
            let string1: String = String::from("Group ");
            let string2: String = String::from("Seven");
            // Boolean Data Type
            let is_true: bool = true;
            let is_false: bool = false;
        
            // Integer Data Manipulation
            let sum = integer1 + integer2;
            let product = integer1 * integer2;
        
            // Floating-point Data Manipulation
            let average = (float1 + float2) / 2.0;
        
            // String Data Manipulation
            let concatenated_string = format!("{}{}", string1, string2);
            let replaced_string = string1.replace("Group ", "Team ");
        
            // Boolean Data Manipulations
            let and_result = is_true && is_false;
            let or_result = is_true || is_false;
        
            // Print results
            println!("Sum: {}", sum);
            println!("Product: {}", product);
            println!("Average: {:.2}", average); // {:.2} formats the float to 2 decimal places
            println!("Concatenated String: {}", concatenated_string);
            println!("Replaced String: {}", replaced_string);
            println!("AND Result: {}", and_result);
            println!("OR Result: {}", or_result);
        }
    
    //Program 2 demonstrates the use of 2 major data structures (array and linked list) as well as 2 major control structures (for loop, and if-else). 
    //In this example, an array of “bpms” of songs is created. Once database implementation has occurred, this will be populated by data provided by the user.  
    //A linked list of the most popular songs based on streams bpm is created and populated. A for loop is used on both structures to calculate their average bpms. 
    //Finally, and if statement compares the two averages, and reports that the users average is higher or lower than the popular average
    fn prog_two() -> String {
        //Declare structures for bpms and variables to store average bpm informatoin
        let bpms = [125, 92, 138, 170, 144];
        let mut most_popular_bpms = LinkedList::new();
            most_popular_bpms.push_back(171);
            most_popular_bpms.push_back(96);
            most_popular_bpms.push_back(110);
            most_popular_bpms.push_back(98);
            most_popular_bpms.push_back(90);
        let mut avg_bpm = 0;
        let mut popular_avg_bpm = 0;
    
        //For loop that total the values contained in each portion of the linked list, using the .iter() iterate through the whole linked list
        for bpm in most_popular_bpms.iter() {
            popular_avg_bpm = popular_avg_bpm + bpm;
        }
        popular_avg_bpm = popular_avg_bpm/most_popular_bpms.len();
    
    
        //For loop that total the values contained in each portion of the array, using the .iter() iterate through the whole array
        for bpm in bpms.iter() {
            avg_bpm = avg_bpm + bpm;
        }
        avg_bpm = avg_bpm/bpms.len();
    
    
        //If statement to determine what result is reported to the user. Compares the users average bpm to the avg of the most popular songs
        //to determine which is higher, and returns a string indicating the result.
        if avg_bpm > popular_avg_bpm {
            return format!("{} {} {}", "Your BPM of", avg_bpm ,"is Higher than Most");
        } else {
            return format!("{} {} {}", "Your BPM of", avg_bpm, "is Lower than Most");
        }
    
    }
    
    //Program 3 demonstrates exception handling. Rust allows the use of the Result enum type to handle
    //recoverable errors. In this example, the outer function takes a reference to a string literal as
    //an argument and returns a string detailing the result of passing that reference to an inner
    //function.
    fn prog_three(s: &str) -> String {
        // Returns a result containing an error if the string contains anything other than uppercase or lowercase letters and containing nothing otherwise.
        // The result return type takes two generic parameters, T and E, where T is the type stored in Ok and E is the type stored in Err.
        fn alphabetical_or_err(input: &str) -> Result<(),String> {
            for b in input.chars() {
                if (b < 'A' || b > 'Z') && (b < 'a' || b > 'z') {
                    return Err(format!("Non-alphabetical! Contains '{:}'", b));
                }
            }
            Ok(())
        }
    
        // The Result type has to be unwrapped in some way. One option is via pattern matching, that is
        // what is done here.
        match alphabetical_or_err(s) {
            Ok(()) => format!("No error for string: {:}", s),
            Err(e) => format!("Err: {:?} for string {:}", e, s)
        }
    }
    
    
    fn main() {
        // Running and/or testing program one
        prog_one();
        // Running and/or testing program two
        println!("{:}",prog_two());
        // Running and/or testing program three
        println!("{:}",prog_three("thisisok"));
        println!("{:}",prog_three("this is not ok"));
        println!("{:}",prog_three("thisisn0t0k"));

}