fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
 
    //* Mutilple Input Ex: 3 3
    let list_input: Vec<_> = input
         .trim()
         .split_whitespace()
         .map(|x| x.parse::<i32>().expect("Not an Integer!"))
         .collect();
    
    println!("{}", list_input[1]*2 - list_input[0])
       
 }