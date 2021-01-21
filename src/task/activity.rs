use chrono::NaiveTime;
use crate::task::utils::time_hm;


#[derive(Debug)]
pub enum ActivityError {
  ParsingError,
  IllegalTimespanError,
}

pub type ActivityResult = Result<f64, ActivityError>;

pub struct Activity {
  description: String,
  start: NaiveTime,
  end: NaiveTime
}

impl Activity {

  pub fn with (descrition: &String, start: &NaiveTime, end: &NaiveTime) -> Activity {
    Activity{description: descrition.clone(), start: start.clone(), end: end.clone()}
  }

  pub fn duration (&self) -> ActivityResult {
    let span = self.end.signed_duration_since(self.start());
    let h = (span.num_minutes() / 60) as f64;
    let m = ((span.num_minutes() % 60) as f64) * 0.01666666667;
    if h + m < 0.0 {
      return Err(ActivityError::IllegalTimespanError);
    }
    Ok(h + m)
  }
  
  pub fn description (&self) -> String {
    self.description.clone()
  }

  pub fn start (&self) -> NaiveTime {
    self.start.clone()
  }

  pub fn end (&self) -> NaiveTime {
    self.end.clone()
  }

  pub fn clone (&self) -> Activity {
    Activity {description: self.description().clone(), start: self.start(), end: self.end()}
  }
}

pub fn parse_activity_line (line: &str) -> Activity {
    let tokens: Vec<&str> = line.splitn(3, ' ').collect();
    Activity::with(
      &String::from(tokens[2]), 
      &time_hm(&String::from(tokens[0])), 
      &time_hm(&String::from(tokens[1]))
    )
}

pub fn str_duration (activity: &Activity) -> Result<String, ActivityError> {
  let span = activity.end().signed_duration_since(activity.start());
  if span.num_minutes() < 0 {
    return Err(ActivityError::IllegalTimespanError);
  }
  let h = (span.num_minutes() / 60) as f64;
  let m = ((span.num_minutes() % 60) as f64) * 0.01666666667;
  Ok(format!("{:.2}", (h + m).abs()))
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn should_calculate_actitivity_duration () {
    let mut activity = Activity::with(&"test".to_string(), &NaiveTime::from_hms(17,0,0), &NaiveTime::from_hms(18,0,0));
    assert_eq!(str_duration(&activity).ok(), Some("1.00".to_owned()));

    activity = Activity::with(&"test".to_string(), &NaiveTime::from_hms(23,0,0), &NaiveTime::from_hms(0,0,0));
    assert_eq!(str_duration(&activity).ok(), Some("23.00".to_owned()));
  }

  #[test]
  fn should_calculate_activity_duration_with_minutes () {
    let mut activity = Activity::with(&"test".to_string(), &NaiveTime::from_hms(17,0,0), &NaiveTime::from_hms(18,45,0));
    assert_eq!(str_duration(&activity).ok(), Some("1.75".to_owned()));

    activity = Activity::with(&"test".to_string(), &NaiveTime::from_hms(17,30,0), &NaiveTime::from_hms(18,00,0));
    assert_eq!(str_duration(&activity).ok(), Some("0.50".to_owned()));
  }

  #[test]
  fn should_calculate_activity_duration_of_five_minutes () {
    let activity = Activity::with(&"test".to_string(), &NaiveTime::from_hms(10,20,0), &NaiveTime::from_hms(10,25,0));
    assert_eq!(str_duration(&activity).ok(), Some("0.08".to_owned()));
  }
}
