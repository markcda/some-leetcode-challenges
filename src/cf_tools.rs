use std::io;
use std::str::FromStr;
use crate::tools::{parse, parse_mul, TResult};

/// Считывает значение с клавиатуры.
#[allow(dead_code)]
pub fn read<T: FromStr>() -> TResult<T> {
  let mut input_string = String::new();
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse::<T>(&input_string)
}

/// Считывает много значений с клавиатуры.
#[allow(dead_code)]
pub fn read_mul<T: FromStr>() -> TResult<Vec<T>> {
  let mut input_string = String::new();
  if io::stdin().read_line(&mut input_string).is_err() { return Err("Не удалось считать строку".into()) }
  parse_mul::<T>(&input_string, Some(" "))
}