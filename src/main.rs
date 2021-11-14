fn main() {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();
   
   let dist_input: Vec<_> = input
   .trim()
   .split_whitespace()
   .map(|x| x.parse::<f64>().expect("Not an Integer!"))
   .collect();

   let a = dist_input[0] * dist_input[0];
   let b = dist_input[1] * dist_input[1];
   let c = a + b;
   println!("{:.6}", (c as f64).sqrt());
   
}