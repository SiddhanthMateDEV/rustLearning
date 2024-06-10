fn main() {
    let number = 3;
    if number < 3 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    if number!=0 {
        println!("Number is present");
    }

    if number%3==0{
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else if number % 1 == 0 {
        println!("Number is divisible by 1");
    } 

    let conidtion = true;
    let number = if conidtion {5} else {0};
    println!("The number is: {number}");

    let conidtion = true;
    let number = if conidtion {"hi"} else {"str"};
    println!("The number is: {number}");

    // loop{
    //     println!("again");
    // }

    let mut counter = 0;
    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };
    println!("The counter is: {counter}");
    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count: {count}");

    let mut _num = 3;

    while _num != 0 {
        println!("{_num}");
        _num -= 1;
    } 

    println!("LIFTOFF!!!");

    let z = [10,20,30,40,50,60];
    let mut index = 0;

    while index<5 {
        println!("The value is: {}",z[index]);
        index+=1;
    }

    println!("\n");

    for element in z {
        println!("The value is: {element}");
    }

    println!("\n");

    for number in (1..4).rev() {
        println!("The value is: {}",number);
    }
}
