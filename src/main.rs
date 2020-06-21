extern crate work;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use work::task::activity::*;
use work::task::work::Day;

fn filename(args: &[String]) -> Option<&str> {
    if args.len() == 2 {
        return Some(&args[1]);
    }
    return None;
}

fn help() -> String {
    String::from("Missing filename! Please invoke: $ ./work [filename]")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = filename(&args);

    if fname.is_none() {
        println!("{}", help());
        return;
    }

    let mut file = File::open(fname.unwrap()).expect("file not found!");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("something went wrong reading the file.");

    let _activities: Vec<Activity> = content
        .trim_end_matches('\n')
        .lines()
        .map(|l| work::task::activity::parse_activity_line(&l))
        .collect();
    let mut day = Day::new();

    day.load(_activities);
    println!("{}", day.sum_report());
}
