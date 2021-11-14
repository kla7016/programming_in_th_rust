fn main() {
    let mut m_n_input = String::new();
    std::io::stdin().read_line(&mut m_n_input).unwrap();
 
    //* Mutilple Input Ex: 3 3
    let dist_input: Vec<_> = m_n_input
         .trim()
         .split_whitespace()
         .map(|x| x.parse::<i32>().expect("Not an Integer!"))
         .collect();
    let m = dist_input[0];
    let n = dist_input[1];
   
    let mut matrix_1: Vec<i32> = vec![];
    for _i in 0..m {
       let mut input_number = String::new();
       std::io::stdin().read_line(&mut input_number).unwrap();
       let list_input_number: Vec<_> = input_number
         .trim()
         .split_whitespace()
         .map(|x| x.parse::<i32>().expect("Not an Integer!"))
         .collect();
       for number in list_input_number {
          matrix_1.push(number)
       }
    }
 
    let mut matrix_2: Vec<i32> = vec![];
    for _i in 0..m {
       let mut input_number = String::new();
       std::io::stdin().read_line(&mut input_number).unwrap();
       let list_input_number: Vec<_> = input_number
         .trim()
         .split_whitespace()
         .map(|x| x.parse::<i32>().expect("Not an Integer!"))
         .collect();
       for number in list_input_number {
          matrix_2.push(number)
       }
    }
 
    // println!("{:?}", matrix_1);
 
    let mut list_result: Vec<i32> = vec![];
    for i in 0..matrix_1.len() {
       let result = matrix_1[i] + matrix_2[i];
       list_result.push(result);
    }
 
    let mut temp_n = 1;
    for r in list_result {
       print!("{}", r);
       if temp_n == n {
          println!("");
          temp_n = 1;
       } else {
          print!(" ");
          temp_n += 1;
       }
    }
 
    // println!("{:?}", list_result);
 }
 