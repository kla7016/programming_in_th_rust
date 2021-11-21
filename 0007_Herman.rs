fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    const PI: f64 = 3.14159265358979323846264338327950288f64;
 
    let input_int = input.trim().parse::<f64>().unwrap();
    let r = input_int * input_int;
    println!("{:.6}", r * PI );
    println!("{:.6}", r * 2.0 );
 }