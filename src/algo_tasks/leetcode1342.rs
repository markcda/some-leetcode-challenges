use crate::tools::{MResult, println, read};

pub fn number_of_steps(mut num: i32) -> i32 {
  let mut steps = 0i32;
  while num > 0 {
    steps += 1;
    if num % 2 == 0 { num /= 2; }
    else { num -= 1; }
  }
  steps
}

pub fn leetcode1342_task() -> MResult {
  println("На каждом шаге чётное число уменьшается вдвое, из нечётного - вычитается единица.");
  let num: i32 = read(Some("Введите число:"))?;
  println(&format!("Результат: {}", number_of_steps(num)));
  Ok(())
}