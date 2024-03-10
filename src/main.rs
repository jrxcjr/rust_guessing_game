use std::io; //Input and output library from the OS.

fn main() { // The main function is just like other languages, the starting point of the application
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //When a variable is created , using the let keyword,
    // the equal operator binds it to an immutable reference in memory
    // by default, all variables in rust are immutable, but when the keyword `mut` is used, this lets the variable be mutable.

    // to invoke a method in a class, in this case, the String class, the `::` notation is used.
    // and a method is called upon by using the `method_name()` notation


     

    io::stdin().
    //the io::stdin() is an implementation that returns an instance of std::io::Stdin, which is a type that represents a handle
    // to operate upon the standard input for your terminal, think of it as the Input class in go.
    
    read_line(&mut guess).
    // the read_line function, is a chained method enacted upon the stdin lib to get the user input and append
    // it into a string (without overwriting its' contents)
    // the "&" operand tells the compiler that it is reading a reference to a memory address, giving a way to access this
    // in more parts of the code without the need to clone it in each usage. 
    expect("Failed to Read Line");
    //the readline returns a Result value, that Result is an enum, the purpose of that is to encode error-handling information
    //The variants that are enabled in the Result enum is "Ok" and "Err", the OK is the successfully generated value and
    //Err indicate that the generation of the value has failed, and information on the error.
    //Not using the expect method to treat the Result enum, generates a warning on the code, but that compiles.
    
    println!("You guessed: {guess}");
    //To print variables inside a println you can use the variable wrapped in curly braces "{variable}"
    
}
