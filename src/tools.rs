use std::io::{self, Write};
use std::str::FromStr;
use crate::structs::matrix::{Matrix, MatrixElement};

pub type MResult = Result<(), &'static str>;
pub type TResult<T> = Result<T, &'static str>;

/// Печатает строку с заданным отступом для решаемой задачи.
pub fn println(text: &str) {
  println!("\t{}", text)
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