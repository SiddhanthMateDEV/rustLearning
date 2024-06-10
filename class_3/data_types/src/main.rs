fn main() {
    let guess: u32 = "32".trim().parse().expect("Not a number");
    {
        let guess: u32 = match "52".trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                return;
            },
        };
        println!("The value inside is: {guess}");
    }
    println!("The value of guess is: {guess}");

    let _x = 2.0;

    let _y: f32 = 3.0;

    let _sum = 5+10;

    let _difference = 95.5-4.3;

    let _product = 4*30;

    let _quotient = 56.7/32.2;

    let _truncated = -5/3;

    let _remained = 43%5;

    let _t = true;

    let _f: bool = false;

    let _c = 'z';

    let _z: char = 'Z';

    let _tup: (i32,f64,u8) = (500,6.4,1);

    let (_a,_b,_c) = _tup;

    println!("The value of b is: {_b}");

    let five_hundred = _tup.0;
    println!("The value of 'five_hundred' is: {five_hundred}");

    let _array_numbers = [1,2,3,4,5,6];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];

    let _first = _array_numbers[0];
    let _second_month = _months[2];

    println!("The first value in '_array_numbers': {_first}");
    println!("The first value in '_second_month': {_second_month}");
}

// Result for the above should be something like this: 
// The value inside is: 52
// The value of guess is: 32
// The value of b is: 6.4
// The value of 'five_hundred' is: 500
// The first value in '_array_numbers': 1
// The first value in '_second_month': March