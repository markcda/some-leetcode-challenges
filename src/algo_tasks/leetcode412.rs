use crate::tools::{MResult, read, println};

/// Функция принимает число `n` и возвращает вектор чисел от 1 до n, где:
///
/// 1. `answer[i] == "FizzBuzz"`, если i делится на 3 и 5
/// 2. `answer[i] == "Fizz"`, если i делится на 3
/// 3. `answer[i] == "Buzz"`, если i делится на 5
/// 4. `answer[i] == i` (как строка) в противном случае
pub fn fizz_buzz(n: i32) -> Vec<String> {
  let mut vec: Vec<String> = vec![];
  for i in 1..(n + 1) {
    if (i % 3 == 0) && (i % 5 == 0) {
      vec.push("FizzBuzz".to_string());
    } else if i % 3 == 0 {
      vec.push("Fizz".to_string());
    } else if i % 5 == 0 {
      vec.push("Buzz".to_string());
    } else {
      vec.push(i.to_string());
    }
  }
  vec
}

pub fn leetcode412_task() -> MResult {
  let n: i32 = read(Some("Введите любое число, большее нуля:"))?;
  if n <= 0 {
    return Err("Число не больше нуля.")
  }
  println(&format!("Результат: {:?}", fizz_buzz(n)));
  Ok(())
}
