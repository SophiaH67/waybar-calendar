use serde_json::json;

fn main() {
  let waybar = generate_waybar("test", "monkey", "tool", "yeah", 23);
  println!("{}", waybar.to_string());
}

fn generate_waybar(text: &str, alt: &str, tooltip: &str, class: &str, percentage: u8) -> serde_json::Value {
  return json!({
    "text": text,
    "alt": alt,
    "tooltip": tooltip,
    "class": class,
    "percentage": percentage
  });
}