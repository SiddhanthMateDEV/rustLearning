fn main() {
    println!("Hello, world!");
    another_function();
    some_number(5);
    value_char(20,'s');

    // let x = (let y = 8); error
    let x = 2;
    let x = {
        let y = 8;
        y+1
    };

    // println!("The value of y is: {y}");Error
    println!("The value of x is: {x}");
    
    let a = five();
    println!("The value of a is: {a}");
    
    let b = plus_one(a);
    println!("The value of b is: {b}");
}

fn plus_one(x: i32)->i32{
    x+1
}

fn five()->i32{
    5
}

fn another_function(){
    println!("Another function.")
}

fn some_number(x: i32){
    println!("The vale of x is: {x}");
}

fn value_char(value: i32, unit_label: char){
    println!("The vale of value is: {value} and char is: {unit_label}");
}