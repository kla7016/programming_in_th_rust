fn main() {
    let mut total_input = String::new();
    std::io::stdin().read_line(&mut total_input).unwrap();
 
    let total_input_int = total_input.trim().parse::<i32>().unwrap();
 
    let mut vector: Vec<i32> = vec![];
 
    for _i in 0..total_input_int{
       let mut input_number = String::new();
       std::io::stdin().read_line(&mut input_number).unwrap();
       let input_number_int = input_number.trim().parse::<i32>().unwrap();
       vector.push(input_number_int)
    }  
    let min_value = vector.iter().min();
    match min_value {
       None => println!("Min value was not found"),
       Some(i) => println!("{}", i)
    }
    let max_value = vector.iter().max();
    match max_value {
       None => println!("Max value was not found"),
       Some(i) => println!("{}", i)
    }
 }
 