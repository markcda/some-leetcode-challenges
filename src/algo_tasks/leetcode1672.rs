use crate::structs::matrix::Matrix;
use crate::tools::{MResult, println, read_matrix};

/// Считает максимальную сумму подмассивов.
pub fn maximum_wealth(accounts: Matrix<i32>) -> i32 {
  accounts.iter()
    .map(|row| row.iter().sum::<i32>())
    .max_by(|x, y| x.cmp(y)).unwrap()
}

pub fn leetcode1672_task() -> MResult {
  println("Введите целочисленную матрицу, после ввода пропустите одну строку:");
  let m = read_matrix()?;
  println(&format!("Результат: {}", maximum_wealth(m)));
  Ok(())
}