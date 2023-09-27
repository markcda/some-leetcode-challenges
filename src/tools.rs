use std::io::{self, Write};
use std::str::FromStr;
use crate::structs::matrix::{Matrix, MatrixElement};

pub type MResult = Result<(), &'static str>;
pub type TResult<T> = Result<T, &'static str>;

/// Печатает строку с заданным отступом для решаемой задачи.
pub fn println(text: &str) {
  println!("\t{}", text)
}

/// Печатает текст без перевода строки на экране.
pub fn print(text: &str) -> MResult {
  print!("{}", text);
  if io::stdout().flush().is_err() { return Err("Не удалось отправить строку".into()) };
  Ok(())
}

/// Преобразовывает часть строки в заданный тип, для которого определён порядок преобразования (`T: FromStr`).
pub fn parse<T: FromStr>(s: &str) -> TResult<T> {
  match s.trim().parse::<T>() {
    Ok(val) => Ok(val),
    Err(_) => Err("Не удалось преобразовать строку к заданному типу".into())
  }
}

/// Создаёт контейнер и, разбивая входную строку на части по разделителю `delimeter`, кладёт в контейнер преобразованные в `T` элементы.
pub fn parse_mul<T: FromStr>(s: &str, delimeter: Option<&str>) -> TResult<Vec<T>> {
  let mut container = Vec::new();
  let delimeter = match delimeter {
    Some(delimeter) => delimeter,
    None => " ".into(),
  };
  for token in s.split(&delimeter).filter(|v| !v.is_empty()) {
    match parse::<T>(token) {
      Ok(val) => container.push(val),
      Err(_) => { return Err("Не удалось преобразовать часть строки к заданному типу"); }
    }
  }
  Ok(container)
}

/// Считывает значение `u16` с клавиатуры.
pub fn read_opt() -> TResult<u16> {
  let mut input_string = String::new();
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse::<u16>(&input_string)
}

/// Считывает значение с клавиатуры с заданным отступом и преобразовывает в `T`.
pub fn read<T: FromStr>(prompt: Option<&str>) -> TResult<T> {
  let mut input_string = String::new();
  if prompt.is_some() { print!("\t{} ", prompt.unwrap()); }
  else { print!("\t"); }
  if io::stdout().flush().is_err() { return Err("Не удалось отправить строку".into()) };
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse::<T>(&input_string)
}

/// Считывает много значений одной строкой с заданным отступом с клавиатуры и преобразовывает в `Vec<T>`.
pub fn read_mul<T: FromStr>(prompt: Option<&str>, delimeter: Option<&str>) -> TResult<Vec<T>> {
  let mut input_string = String::new();
  if prompt.is_some() { print!("\t{} ", prompt.unwrap()); }
  else { print!("\t"); }
  if io::stdout().flush().is_err() { return Err("Не удалось отправить строку".into()) };
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse_mul::<T>(&input_string, delimeter)
}

/// Считывает матрицу.
pub fn read_matrix<T: MatrixElement>() -> TResult<Matrix<T>> {
  let mut strs: Vec<String> = Vec::new();
  while let Ok(s) = read::<String>(None) {
    if !s.is_empty() { strs.push(s); }
    else { break }
  }
  Ok(Matrix::parse_from_lines(strs)?)
}

#[cfg(test)]
mod tests {
  use crate::tools::parse;
  use crate::tools::parse_mul;

  #[test]
  fn test_parse_single() {
    assert_eq!(parse::<u8>("0"), Ok(0u8));
    assert_eq!(parse::<i32>("1"), Ok(1i32));
    assert!(parse::<u8>("-1").is_err());
    assert!(parse::<i32>("a").is_err());
    assert!(parse::<i32>("-").is_err());
  }

  #[test]
  fn test_parse_multiple() {
    assert_eq!(parse_mul::<u8>("7 11 8 6 3 8 9", None), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
    assert_eq!(parse_mul::<u8>("7,11,8,6,3,8,9", Some(",".into())), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
    assert_eq!(parse_mul::<u8>("7 11 8 6     9", None), Ok(vec![7u8, 11u8, 8u8, 6u8, 9u8]));
  }
}
