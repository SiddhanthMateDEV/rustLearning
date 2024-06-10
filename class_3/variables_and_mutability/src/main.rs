fn main() {

    // This would give errors 
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    //if a variable needs to be changed then it should be initialised with 'mut'
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 6;

    let y = y+1;

    {
        let y = y*2;
        println!("The value of y in the inner scope: {y}");
    }
    println!("The value of y in the  scope: {y}");
}
