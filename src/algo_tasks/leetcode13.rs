use crate::tools::{MResult, TResult, println, read};

/// Функция для конвертации чисел в римской нотации в числа в арабской нотации.
fn roman_to_int(s: String) -> TResult<i32> {
  let mut n: i32 = 0;
  let mut iter = s.chars().zip(s.chars().skip(1));
  while let Some(pair) = iter.next() {
    match pair.0 {
      'I' => match pair.1 {
        'V' | 'X' => n -= 1,
        _ => n += 1,
      },
      'V' => n += 5,
      'X' => match pair.1 {
        'L' | 'C' => n -= 10,
        _ => n += 10,
      },
      'L' => n += 50,
      'C' => match pair.1 {
        'D' | 'M' => n -= 100,
        _ => n += 100,
      },
      'D' => n += 500,
      'M' => n += 1000,
      _ => {
        return Err("Не удалось сконвертировать число, т.к. оно содержит неизвестные символы.".into())
      },
    }
  }
  match s.chars().last().unwrap() {
    'I' => n += 1,
    'V' => n += 5,
    'X' => n += 10,
    'L' => n += 50,
    'C' => n += 100,
    'D' => n += 500,
    'M' => n += 1000,
    _ => {
      return Err("Не удалось сконвертировать число, т.к. оно содержит неизвестные символы.".into())
    },
  }
  Ok(n)
}

pub fn leetcode13_task() -> MResult {
  let roman_num: String = read(Some("Введите число в римской нотации:".into()))?;
  match roman_to_int(roman_num) {
    Ok(arabic_num) => println(&format!("Результат: {}", arabic_num)),
    Err(err_str) => eprintln!("{}", err_str),
  }
  Ok(())
}
