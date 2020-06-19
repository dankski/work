extern crate chrono;

use chrono::NaiveTime;
use std::error::Error;

pub fn time_hm(time: &String) -> NaiveTime {
  match NaiveTime::parse_from_str(time, "%H:%M") {
    Ok(NaiveTime) => NaiveTime,
    Err(error) => panic!(
      "There was a problem parsing the time: {}",
      error.to_string()
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn should_panic_on_invalid_characters() {
    let time = String::from("15.30");
    time_hm(&time);
  }

  #[test]
  fn should_parse_naive_time() {
    let time = String::from("15:30");
    assert_eq!(time_hm(&time), NaiveTime::from_hms(15, 30, 0));
  }

  #[test]
  fn should_return_time_span() {
    let start = time_hm(&String::from("15:30"));
    let end = time_hm(&String::from("16:01"));
    let span = end.signed_duration_since(start);
    assert_eq!(span.num_minutes(), 31);
  }
}
