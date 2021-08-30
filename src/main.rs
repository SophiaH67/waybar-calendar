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
      &now.format("%b %-d, %H:%M:%S").to_string(),
      "calendar",
    );

    sleep(time::Duration::from_millis(1000));
  }
}

fn generate_waybar(
  text: &str,
  tooltip: &str,
  class: &str,
) {
  let waybar = json!({
    "text": text,
    "tooltip": tooltip,
    "class": class,
  });
  println!("{}", waybar.to_string());
}
