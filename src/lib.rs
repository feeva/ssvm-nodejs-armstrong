use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn armstrong(n: &str) -> i32 {
  let mut num: u64 = n.parse().unwrap();
  let len = n.len();
  let mut sum: u64 = 0;

  loop {
    let digit = num % 10;
    if digit == 0 {
      break;
    }

    num /= 10;
    sum += digit.pow(len as u32);
  }

  println!("armstrong({}) = {}", n, sum);

  return (n == sum.to_string()) as i32;
}
