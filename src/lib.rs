use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn quotes() -> (&'static str, &'static str) {
  println!("You've asked for a quote!");
  
  let quotes = vec![("The universe is change; our life is what our thoughts make it.", "Marcus Aurelius"),
  ("God save me from my friends. I can protect myself from my enemies.", "Claude Louis Hector de Villars "),
  ("The soul becomes dyed with the color of its thoughts.", "Marcus Aurelius"),
  ("The price of anything is the amount of life you exchange for it.", "David Thoreau"),
  ("Life is like a landscape. You live in the midst of it but can describe it only from the vantage point of distance. ", "Charles Lindbergh"),
  ("It is not death that a man should fear, but he should fear never beginning to live.", "Marcus Aurelius")];
 
  let rng = rand::thread_rng().gen_range(0,6);

  return quotes[rng];
}
