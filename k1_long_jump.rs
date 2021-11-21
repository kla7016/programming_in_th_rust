fn main() {
    let mut input_1 = String::new();
    std::io::stdin().read_line(&mut input_1).unwrap();
    let list_input_1: Vec<_> = input_1
       .trim()
       .split_whitespace()
       .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       .collect();
 
    let mut input_2 = String::new();
    std::io::stdin().read_line(&mut input_2).unwrap();
    let list_input_2: Vec<_> = input_2
       .trim()
       .split_whitespace()
       .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       .collect();
 
    let mut input_3 = String::new();
    std::io::stdin().read_line(&mut input_3).unwrap();
    let list_input_3: Vec<_> = input_3
       .trim()
       .split_whitespace()
       .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       .collect();
 
    let n = list_input_1[2];
    for _i in 0..n {
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).unwrap();
       // let list_input: Vec<_> = input
       // .trim()
       // .split_whitespace()
       // .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       // .collect();
    }
    println!("-1");
 }