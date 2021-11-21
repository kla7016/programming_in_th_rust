use std::collections::btree_set::{BTreeSet, IntoIter};

fn main() {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).unwrap();

   let input_int = input.trim().parse::<i32>().unwrap();

   let mut data_i: Vec<i32> = vec![];
   let mut data_j: Vec<i32> = vec![];

   // let mut result = 0;
   for _i in 0..input_int {
      let mut input_detail = String::new();
      std::io::stdin().read_line(&mut input_detail).unwrap();

      let list_input: Vec<_> = input_detail
      .trim()
      .split_whitespace()
      .map(|x| x.parse::<i32>().expect("Not an Integer!"))
      .collect();
      data_i.push(list_input[0]);
      data_j.push(list_input[1]);
   }

   // for i in data_i.iter() {
   //    println!("{}", i);
   // }
   let items = vec![0, 0, 1, 2];
   for perm in items.iter().permutations(items.len()).unique() {
       println!("{:?}", perm);
   }
      
}