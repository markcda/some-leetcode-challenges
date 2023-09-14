use std::io::{self, Write};
use std::str::FromStr;

pub type MResult = Result<(), String>;
pub type TResult<T> = Result<T, String>;

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
pub fn parse_mul<T: FromStr>(s: &str, delimeter: Option<String>) -> TResult<Vec<T>> {
  let mut container = Vec::new();
  let delimeter = match delimeter {
    Some(delimeter) => delimeter,
    None => " ".into(),
  };
  for token in s.split(&delimeter) {
    match parse::<T>(token) {
      Ok(val) => container.push(val),
      Err(_) => { return Err(format!("Не удалось преобразовать часть строки к заданному типу (token = {})", token)); }
    }
  }
  Ok(container)
}

/// Считывает значение с клавиатуры и преобразовывает в `T`.
pub fn read_opt() -> TResult<u8> {
  let mut input_string = String::new();
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse::<u8>(&input_string)
}

/// Считывает значение с клавиатуры с заданным отступом и преобразовывает в `T`.
pub fn read<T: FromStr>(prompt: Option<String>) -> TResult<T> {
  let mut input_string = String::new();
  if prompt.is_some() { println(&format!("{}", prompt.unwrap())); }
  print!("\t");
  if io::stdout().flush().is_err() { return Err("Не удалось отправить строку".into()) };
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse::<T>(&input_string)
}

/// Считывает много значений одной строкой с заданным отступом с клавиатуры и преобразовывает в `Vec<T>`.
pub fn read_mul<T: FromStr>(prompt: Option<String>, delimeter: Option<String>) -> TResult<Vec<T>> {
  let mut input_string = String::new();
  if prompt.is_some() { println(&format!("{}", prompt.unwrap())); }
  print!("\t");
  if io::stdout().flush().is_err() { return Err("Не удалось отправить строку".into()) };
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse_mul::<T>(&input_string, delimeter)
}
