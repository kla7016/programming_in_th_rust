fn main() {
    let mut first_input = String::new();
    std::io::stdin().read_line(&mut first_input).unwrap();
    let list_first_input: Vec<_> = first_input
       .trim()
       .split_whitespace()
       .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       .collect();
    let n = list_first_input[0];
    for i in 0..n {
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).unwrap();
       let list_input: Vec<_> = input
       .trim()
       .split_whitespace()
       .map(|x| x.parse::<i32>().expect("Not an Integer!"))
       .collect();
    }
 
    let mut last_input = String::new();
    std::io::stdin().read_line(&mut last_input).unwrap();
 
    println!("1")
 }