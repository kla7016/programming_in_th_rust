use std::collections::btree_set::{BTreeSet, IntoIter};

enum UniquePermutations {
    Leaf {
        elements: Option<Vec<i32>>,
    },
    Stem {
        elements: Vec<i32>,
        unique_elements: IntoIter<i32>,
        first_element: i32,
        inner: Box<Self>,
    },
}

impl UniquePermutations {
    fn new(elements: Vec<i32>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone())
                .expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(
        mut unique_elements: impl Iterator<Item = i32>,
        elements: Vec<i32>,
    ) -> Option<(i32, Box<Self>)> {
        let first_element = unique_elements.next()?;

        let mut remaining_elements = elements;

        if let Some(idx) = remaining_elements.iter().position(|&i| i == first_element) {
            remaining_elements.remove(idx);
        }

        let inner = Box::new(Self::new(remaining_elements));

        Some((first_element, inner))
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                match inner.next() {
                    Some(mut v) => {
                        v.insert(0, *first_element);
                        return Some(v);
                    }
                    None => {
                        let (next_fe, next_i) =
                            Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}

fn main() {
   let mut input_1 = String::new();
   std::io::stdin().read_line(&mut input_1).unwrap();
   let list_input_1: Vec<_> = input_1
      .trim()
      .split_whitespace()
      .map(|x| x.parse::<i32>().expect("Not an Integer!"))
      .collect();

   let n = list_input_1[0];
   let ran = list_input_1[1];
   let minus = list_input_1[2];
   let mut list_input = vec![];
   for _i in 0..n {
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).unwrap();
      let list_temp: Vec<_> = input
      .trim()
      .split_whitespace()
      .map(|x| x.parse::<i32>().expect("Not an Integer!"))
      .collect();
      list_input.push(list_temp);
   }
   let mut input_2 = String::new();
   std::io::stdin().read_line(&mut input_2).unwrap();
   let list_input_2: Vec<_> = input_2
      .trim()
      .split_whitespace()
      .map(|x| x.parse::<i32>().expect("Not an Integer!"))
      .collect();
   
   
   // let mut list_router = vec![];
   let mut sum = 0;
   if minus == 0 {
      for i in 0..list_input_2.len() {
         if i == list_input_2.len() - 1 {
            break;
         }
         let num: usize = list_input_2[i] as usize;
         let num_n: usize = list_input_2[i+1] as usize;
         // println!("{}", list_input[num - 1][num_n - 1]);
         sum = sum + list_input[num - 1][num_n - 1] 
      }
   } else {
      // let mut list_ran = vec![];
      // for i in 0..ran {
      //    list_ran.push(i);
      // }
      // for perm in UniquePermutations::new(list_ran) {
      //    let mut temp_list_input_2 = list_input_2.clone(); 
      //    let mut list_remove = vec![];
      //    for j in 0..minus {
      //       let jj: usize = j as usize;
      //       // let mut a = perm[jj] - j;
      //       let index: usize = perm[jj] as usize;
      //       list_remove.push(index);
      //       // temp_list_input_2.retain(|&x| x != perm[jj]);
      //    }
      //    remove_multiple(&mut temp_list_input_2, &list_remove);
      //    let mut temp_sum = 0;
      //    for i in 0..temp_list_input_2.len() {
      //       if i == temp_list_input_2.len() - 1 {
      //          break;
      //       }
      //       let num: usize = temp_list_input_2[i] as usize;
      //       let num_n: usize = temp_list_input_2[i+1] as usize;
      //       // println!("{}", list_input[num - 1][num_n - 1]);
      //       temp_sum = temp_sum + list_input[num - 1][num_n - 1];
      //    }
      //    if sum == -1 {
      //       sum = temp_sum;
      //    } else {
      //       if temp_sum < sum {
      //          sum = temp_sum;
      //       }
      //    }
      // }
   }

   println!("{}", sum);
}

fn remove_multiple<T>(source: &mut Vec<T>, indices_to_remove: &[usize]) -> Vec<T> {
   indices_to_remove.iter()
       .copied()
       .map(|i| source.swap_remove(i))
       .collect()
}