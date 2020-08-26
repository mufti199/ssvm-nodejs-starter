use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  let t = "\nI hope you are having a great day!"
  return r + s;
}
