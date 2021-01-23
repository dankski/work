use crate::task::activity::str_duration;
use crate::task::activity::{Activity};
use std::collections::HashMap;

pub struct Day {
  activities: Vec<Activity>,
}

impl Day {
  pub fn new() -> Day {
    Day {
      activities: Vec::new(),
    }
  }

  pub fn load(&mut self, activities: Vec<Activity>) {
    activities
      .iter()
      .for_each(|a| self.activities.push(a.clone()));
  }

  pub fn simple_report(&self) -> String {
    let mut sp = String::new();
    sp.push_str("--------------------------------------------------------------\n");
    sp.push_str("Simple Report\n");
    sp.push_str("--------------------------------------------------------------\n");
    self
      .activities
      .iter()
      .for_each(|a| sp.push_str(&format!("{}\t{}\n", str_duration(&a).unwrap(), a.description())));
    sp.push_str("--------------------------------------------------------------\n\n");
    sp.clone()
  }

  pub fn sum_report(&self) -> String {
    let mut sp = String::new();
    sp.push_str(&self.simple_report());
    sp.push_str("Summarized\n");
    sp.push_str("--------------------------------------------------------------\n");
    sp.push_str(&self.aggregate_activities());
    sp.push_str("--------------------------------------------------------------\n");
    sp.push_str(&self.total_hours());
    sp.push_str("--------------------------------------------------------------\n");
    sp.clone()
  }

  fn aggregate_activities(&self) -> String {
    let mut hm: HashMap<String, f64> = HashMap::new();
    for a in self.activities.iter() {
      if hm.contains_key(&a.description()) {
        let v = hm.get(&a.description()).unwrap_or(&0.0);
        let dur: f64 = match a.duration() {
            Ok(dur) => dur,
            Err(why) => panic!("{:?}", why)
        };
        let sum = v + dur;
        hm.insert(a.description(), sum);
      } else {
        let dur: f64 = match a.duration() {
          Ok(dur) => dur,
          Err(why) => panic!("{:?}", why)
        };
        hm.insert(a.description(), dur);
      }
    }

    let mut agg = String::new();

    let duration_converter = |dur: f64| -> String {
      let mut value: String = format!("{:.3}", dur);
      value.truncate(value.len()-1);
      value
    };

    hm.iter()
      .for_each(|(k, v)| agg.push_str(format!("{}\t{}\n", duration_converter(*v), k).as_str()));

    agg.clone()
  }

  fn total_hours(&self) -> String {
    let dur: f64 = self.activities.iter().map(|a| a.duration().unwrap()).sum();
    let mut value: String = format!("{:.3}", dur);
    value.truncate(value.len()-1);
    format!("Total:\t{}\n", value)
  }
}
