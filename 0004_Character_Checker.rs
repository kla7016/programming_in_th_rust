fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_trim = input.trim();
    if input_trim == input_trim.to_lowercase() {
       println!("All Small Letter")
    } else if input_trim == input_trim.to_uppercase() {
       println!("All Capital Letter")
    } else {
       println!("Mix")
    }
 }