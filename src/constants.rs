pub fn integer_for_number(text: &str) -> i32 {
  match text {
    "one"   => 1,
    "two"   => 2,
    "three" => 3,
    "four"  => 4,
    "five"  => 5,
    "six"   => 6,
    "seven" => 7,
    "eight" => 8,
    "nine"  => 9,
    "ten"   => 10,
    _       => 0
  }
}

pub fn month_for_number(text: &str) -> i32 {
  match text {
    "january"   => 1,
    "february"  => 2,
    "march"     => 3,
    "april"     => 4,
    "may"       => 5,
    "june"      => 6,
    "july"      => 7,
    "august"    => 8,
    "september" => 9,
    "october"   => 10,
    "november"  => 11,
    "december"  => 12,
    _           => 0
  }
}