// Statements: perform action, do not return value
// Expressions: perform action, return value
fn main() {
  // lets are statements.
  //let y = 6;

  // Thus you can't assign:
  //let x = (let y = 6);

  // Here's how you do it:
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is {}", y);

  // You do a similar thing with functions that return
  // values
  let x = five();
  println!("The value of x is {}", x);

  let z = incr(5);
  println!("The value of z is {}", z);
}

fn five() -> i32 {
  5
}

fn incr(x : i32) -> i32 {
  x + 1

  // If you end it with a semi-colon, the return type
  // is void! This will produce an error
  //x + 1;
}
