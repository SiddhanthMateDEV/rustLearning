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
    
    let x = 5;
    let y = x;

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{},world",s1);
    println!("{},world",s2);
}
