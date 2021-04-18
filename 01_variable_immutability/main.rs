fn main() {
  // Have to add "mut" to modify variable.
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is now: {}", x);

  // Can't mutate at all.
  const MAX_POINTS : u32 = 100_000;
  println!("Max points is: {}", MAX_POINTS);

  // Alternative to mutation: shadowing
  let y = 1;
  let y = y + 1;
  let y = y + 1;
  println!("The value of y is: {}", y);

  // Note that shadowing allows this:
  let z = " ";
  let z = z.len();
  println!("The value of z is: {}", z);

  // But can't do it with mut! The type is
  // permanently set at this point.
  /*let mut w = " ";
  /*let*/ w = w.len();
  println!("The value of w is: {}", w);*/
}
