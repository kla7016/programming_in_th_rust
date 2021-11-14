fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
 
    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).unwrap();
    
    let sum = line.trim().parse::<i32>().unwrap() + line2.trim().parse::<i32>().unwrap();
    
    println!("{}", sum);
 }
 