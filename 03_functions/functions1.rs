fn main() {
  println!("Hello from main");
  another_function();
  func_with_param(5);
}

fn another_function() {
  println!("Hello from another_function");
}

fn func_with_param(x : i32) {
  println!("The value of x is: {}", x);
}
