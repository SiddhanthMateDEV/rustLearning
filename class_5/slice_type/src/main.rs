fn main() {

    //The slice type
    let s1 = String::from("siddhanth");
    let s1_size = str_reader(&s1);
    println!("The size of s1 is: {}",s1_size);

    let mut s2 = String::from("Siddhanth");
    let _word = str_reader(&s2);
    s2.clear();

    println!("s2 After using .clear(): {}",s2);

    //String Slices 
    let s3 = String::from("Hello, World");
    let hello = &s3[0..5];
    let world = &s3[7..12];
    println!("{} {}", hello, world);
    
    let length = s3.len();
    
    let s4 = &s3[2..length];
    println!("{}",s4);
    
    let s5 = &s3[2..];
    println!("{}",s5);

    let _y = first_word(&s3);
    println!("{}",_y);

    let s6 = [1,2,3,4,5,6];
    let slice = &s6[1..3];
    assert_eq!(slice, &[8,9]);
}

fn str_reader(s: &String)->usize{
    let _bytes_str = s.as_bytes();
    println!("the string as_bytes() is: {:?}",_bytes_str);

    for (i, &item) in _bytes_str.iter().enumerate(){
        if item == b'i'{
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String)->&str{
    let bytes_str = s.as_bytes();

    for (i,&item) in bytes_str.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

