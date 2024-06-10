use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number \n");
    // 'thread_rng()' is to create a random number generator instance which 
    // it is local to the current thread 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //Running this iniside a loop till the final number is guessed correctly
    loop {    
        println!("Please input your guess \n");

        //intiialising an empty string here
        let mut guess = String::new();

        //io::stdin() calls the input function
        //read_line() reads a line of string similar to get(line) in c++
        io::stdin().read_line(&mut guess).expect("failed to read line");

        //This part is to convert the string guess to integers
        //the trim() function removes whitespace characters in a string
        //parse() converts the string characters which contain digits to int


        // this was used in guessing_game_3 
        //let guess: u32 = guess.trim().parse().expect("Please type a number");

        //in guessing guessing_game_4 were going to try using match for error matching

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //matches the value by calling the cmp library and comparing the two integers    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
            },
            Ordering::Greater => {
                println!("Too Big");
            },
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    };
    println!("The secret number is: {secret_number} \n");
}
