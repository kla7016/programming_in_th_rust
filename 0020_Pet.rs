fn main() {
    let mut result = 0;
    let mut num_result = 1;
    for i in 0..5 {
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).unwrap();
       let list_input: Vec<_> = input
          .trim()
          .split_whitespace()
          .map(|x| x.parse::<i32>().expect("Not an Integer!"))
          .collect();
       let mut sum = 0;
       for number in list_input {
          sum += number;
       }
       if i == 0 {
          result = sum;
          num_result = i + 1;
       } else {
          if sum > result {
             result = sum;
             num_result = i + 1;
          }
       }
    }
 
    println!("{} {}", num_result, result);       
 }