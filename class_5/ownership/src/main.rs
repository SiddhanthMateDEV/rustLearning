fn main() {
    // Error
    // let x = String::from("hello");
    // x.push_str(",world!");
    // println!("{}",x);

    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{}",s);
    
    {
        let s = String::from("Hello");
        println!("Inner scope has s = {s}");
    }
    
    let _x = 5;
    let _y = _x;

    let s1 = String::from("Hello");

    {
        let s2 = s1.clone();
        println!("{}, World!",s2);
    }

    // println!("{}, world",s2);
    let _s4 = some_string();
    println!("{}, World",_s4);

    let (string_return_,length_str) = some_string_(s1.clone());
    println!("{}, world, the length of the string is {}",string_return_,length_str);


    //Reference example
    let _s6 = String::from("Reference example");
    let _ref_eg = reference_example(&s1);
    println!("This is the output length of the: {}", _ref_eg);

    //reference change using reference
    let mut ref_str = String::from("hello");
    change_reference(&mut ref_str);
    // println!("The change reference output: {}",ref_str);

    // let r1 = &mut ref_str;
    // let r2 = &mut ref_str;

    // println!("The value of r1: {} and r2: {}",r1,r2);
    let x1 = &ref_str;
    let x2 = &ref_str;
    println!("x1: {} and x2: {}",x1,x2);
    {
        let r1 = &mut ref_str;
        println!("The value of r1: {}",r1);
    }
    let r2 = &mut ref_str;

    println!("The value of r2: {}",r2);

    let _reference_to_nothing = dangle();

    // println!("Refernce to nothing: {}",reference_to_nothing);
    // dangle();



}

fn dangle()-> String{
    let s = String::from("Hello");
    s
}

// fn dangle_static()-> &'static String{
//     let _s = String::from("Hello");
//     &_s
// }

fn change_reference(input: &mut String){
    input.push_str(", world");
}

fn some_string_(_input_: String)->(String,usize) {
    let length = _input_.len();
    (_input_,length)
}

fn reference_example(_input_: &String)->usize{
    _input_.len()
}


fn some_string()->String {
    let s3 = String::from("Hello");
    s3     
}



