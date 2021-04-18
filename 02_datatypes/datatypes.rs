use std::io;

fn main() {
  let tup : (i32, f64, u8) = (500, 6.4, 1);
  println!("Tuple contents: {}, {}, {}", tup.0, tup.1, tup.2);

  println!("Please enter a value");
  let arr : [i32; 5] = [1, 2, 3, 4, 5];
  let mut index = String::new();
  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index : usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");
  let elem = arr[index];

  println!("The value of the element at index {} is: {}",
    index, elem);
}
