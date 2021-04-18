fn if_demo() {
  let number : i32 = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}

fn let_if_demo() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  // Gives the error: if and else have incompatible types
  //let number = if condition { 5 } else { "six" };
  //           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  // expected integral variable, found &str.
  // Types of both branches must be same!

  println!("The value of number is: {}", number);
}

fn loop_demo() {
  /*loop {
    println!("again!");
  }*/

  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
      //break;
      // Break statements have a return type which is
      // assigned to result. I love this! In this case
      // "break;" has the unit return type.
    }
  };

  println!("The result is {}", result);
}

fn while_loop_demo() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }

  println!("L I F T O F F!!!");
}

fn for_loop_demo() {
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("the value is: {}", element);
  }

  // Alternative while_loop_demo
  for number in (1..4).rev() {
    println!("{}!", number);
  }

  println!("L I F T O F F!!!");
}

fn main() {
  if_demo();
  let_if_demo();
  loop_demo();
  // By the way, rust will know if you write a function
  // that isn't called, and will warn you!
  while_loop_demo();
  for_loop_demo();
}
