fn main() {
    // this is a comment || 


    /* this is 
    a multi 
    line 
    comment 
     */


    println!("Hello From Rust");

    // learning some basic output commands 
    println!("The Value of the constant is {}",10);

    //learning to print strings 
    println!("My first name is {}, My second name is {}","Exo","Dus");

    // learning the print command 
    print!("this is a print command");
    print!("this is going to be printed on the same line");
    println!("");

    //learning to print on multiple lines using the print command 
    print!("This is going 
            to be printed on muptiple 
    lines ");

    // learning the use of escape sequences
    println!(" \n\n This is going to be printed after two lines. \t This will have a tab ");

    // leanring the uses of \ (backslash)
    println!("This will print single quote \' and this will print double quote \"");
    println!("This is going to print one back slash \\ ");

    print!("this is some text that will be overwritten \r This text will only appear on the screen");


    //learning the positional arguement
    println!("\n  I am doing {2},from {1} years and i {0}", "like", 20, "programming");
}
 