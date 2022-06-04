// you can nest modules in a parent module
// by default all child modules are private to their ancestors
// but ancestors are public to their children

// in order to expose the functionality of a child module
// we have to use the pub keyword for both the child module
// and the function
pub mod hosting {
  pub fn add_to_waitlist() {}

  fn seat_at_table() {}
}

pub mod serving {
  fn take_order() {}

  pub fn serve_order() {}

  fn take_payment() {}
}
