use serde_json::json;

use std::thread::sleep;
use std::time;

use chrono::{DateTime, Duration, Utc};

fn main() {
  loop {
    let mut now: DateTime<Utc> = Utc::now();
    let month = now.format("%m").to_string().parse::<i32>().unwrap();
    if month == 8 {
      now = now + Duration::days(365 * 10 + 2);
    }
    generate_waybar(
      &now.format("%b %e %Y").to_string(),
      "monkey",
      &now.format("%b %-d, %H:%M:%S").to_string(),
      "calendar",
      23,
    );

    sleep(time::Duration::from_millis(1000));
  }
}

fn generate_waybar(
  text: &str,
  alt: &str,
  tooltip: &str,
  class: &str,
  percentage: u8,
) {
  let waybar = json!({
    "text": text,
    "alt": alt,
    "tooltip": tooltip,
    "class": class,
    "percentage": percentage
  });
  println!("{}", waybar.to_string());
}
