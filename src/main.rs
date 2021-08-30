use serde_json::json;

use std::thread::sleep;
use std::time;

use chrono::{DateTime, Datelike, Duration, Utc};
use chrono_tz::Europe::Amsterdam;
use chrono_tz::Japan;

fn main() {
  loop {
    let now = get_now();
    generate_waybar(
      &now.format("%b %e %Y").to_string(),
      &now.format("%b %-d, %H:%M:%S").to_string(),
      "calendar",
    );

    sleep(time::Duration::from_millis(1000));
  }
}

fn get_now() -> DateTime<chrono_tz::Tz> {
  return weebify_time(
    DateTime::from(Utc::now().with_timezone(&Amsterdam))
  );
}

fn weebify_time(in_time: DateTime<chrono_tz::Tz>) -> DateTime<chrono_tz::Tz> {
  let mut new_time: DateTime<chrono_tz::Tz> = in_time;
  let japan_time = in_time.with_timezone(&Japan);

  /* The flower we saw that day */
  if japan_time.month() == 8 { new_time = new_time+(Duration::days(365*10+2)); }
  
  return new_time;
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
