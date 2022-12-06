mod input;

fn group_by_elves(input: &str) -> Vec<Vec<u64>> {
   input.lines().fold(vec![vec![]], |mut acc, line| {
      if line.is_empty() {
          acc.push(vec![]);
      } else {
          acc.last_mut().unwrap().push(line.parse::<u64>().unwrap());
      }
        acc
  })
}

pub fn main() {
   let input_string: String = input::get_input(1);

   let grouped = group_by_elves(&input_string);
   let mut sums: Vec<u64> = grouped.iter().map(|x| x.iter().sum()).collect();
   sums.sort();

   println!("Biggest sum ist: {}", sums.last().unwrap());

let sum_max3: u64 = sums.iter().rev().take(3).sum();
   println!("Sum of the 3 biggest sums is: {}", sum_max3);
}
