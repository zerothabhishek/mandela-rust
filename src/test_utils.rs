
pub fn data_part(msg: String) -> String {
  let result: crate::MandelaMsg =
      serde_json::from_str(msg.as_str()).expect("Failed parsing msg");
  return result.d.clone().unwrap();
}
