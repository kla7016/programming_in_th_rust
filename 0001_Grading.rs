fn main() {
    let mut score1 = String::new();
    std::io::stdin().read_line(&mut score1).unwrap();
 
    let mut score2 = String::new();
    std::io::stdin().read_line(&mut score2).unwrap();
 
    let mut score3 = String::new();
    std::io::stdin().read_line(&mut score3).unwrap();
    
 
    let score1_int = score1.trim().parse::<i32>().unwrap();
    let score2_int = score2.trim().parse::<i32>().unwrap();
    let score3_int = score3.trim().parse::<i32>().unwrap();
 
    let total_score = score1_int + score2_int + score3_int;
 
    if total_score >= 80 && total_score <= 100 {
       println!("A");
    } else if total_score >= 75 && total_score <= 79 {
       println!("B+");
    } else if total_score >= 70 && total_score <= 74 {
       println!("B");
    } else if total_score >= 65 && total_score <= 69 {
       println!("C+");
    } else if total_score >= 60 && total_score <= 64 {
       println!("C");
    } else if total_score >= 55 && total_score <= 59 {
       println!("D+");
    } else if total_score >= 50 && total_score <= 54 {
       println!("D");
    } else {
       println!("F");
    }
    
 }
 