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
   let mut first_input = String::new();
   std::io::stdin().read_line(&mut first_input).unwrap();
   let list_first_input: Vec<_> = first_input
      .trim()
      .split_whitespace()
      .map(|x| x.parse::<i32>().expect("Not an Integer!"))
      .collect();
   let n = list_first_input[0];
   let x = list_first_input[1];
   let y = list_first_input[2];
   let mut items = vec![];
   let mut list_line = Vec::new();
   for i in 0..n {
      items.push(i);
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).unwrap();
      list_line.push(input)
   }
   let mut result = -1;
    for perm in UniquePermutations::new(items) {
       let mut my_x  = 0;
       let mut my_y  = 0;
       let mut tmp_sum = 0;
        for elem in perm {
           let num_usize: usize = elem as usize;
         //   println!("{}", list_line[num_usize]);
           let list_input: Vec<_> = list_line[num_usize]
           .trim()
           .split_whitespace()
           .map(|x| x.parse::<i32>().expect("Not an Integer!"))
           .collect();
           if my_x < x && my_y < y {
              my_x = my_x + list_input[0];
              my_y = my_y + list_input[1];
              tmp_sum = tmp_sum + list_input[2];
           } else if my_x >= x && my_y >= y {
              if result == -1 {
                 result = tmp_sum
              } else {
                 if tmp_sum < result {
                    result = tmp_sum
                 }
              }
           }
        }
    }
    println!("{}", result);
}