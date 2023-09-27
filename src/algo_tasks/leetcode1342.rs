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

#[cfg(test)]
mod tests {
  #[test]
  fn test_number_of_steps_to_reduce() {
    use crate::algo_tasks::leetcode1342::number_of_steps;
    
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
  }
}
