const RECURRING: &'static [&'static str] = &["every", "*"];

// Tell lib.rs to look for a local file called constants.rs
mod constants;

// Email config
pub struct Config {
  pub recurring: bool
}

// Is this a recurring declaration
pub fn is_recurring(prefix: &str) -> bool {
  let recurring = RECURRING.iter().any(|&x| prefix.contains(x));
  println!("Recurring?: {}", recurring);
  return recurring;
}

// Parse the given email address
pub fn parse(email: &str) -> bool {
  // Instantiate this addresses Config
  let mut config = Config { recurring: false };

  // Split and get the first item in the array
  let v: Vec<&str> = email.split("@").collect();
  let prefix = v[0];
  let _domain = v[1];

  // Store all the token parts of the email prefix
  // let tokens = prefix.split("-");

  // FIXME: String concatenation!
  // Change letter representation of numbers to numbers
  //let mut thing = String::with_capacity(20);
  //v.iter().map(|&x| thing.push_str(x) ).collect::<Vec<_>>();

  println!("LOL: {}", constants::integer_for_number("one"));

  config.recurring = is_recurring(&prefix);
  return config.recurring;
}