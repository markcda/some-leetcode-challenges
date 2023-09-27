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

#[cfg(test)]
mod tests {
  #[test]
  fn test_maximum_wealth() {
    use crate::algo_tasks::leetcode1672::maximum_wealth;
    use crate::structs::matrix::Matrix;
    
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![1, 2, 3],
        vec![3, 2, 1]
      ]).unwrap()), 6);
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![1,5],
        vec![7,3],
        vec![3,5]
      ]).unwrap()), 10);
    assert_eq!(maximum_wealth(Matrix::from(
      vec![
        vec![2,8,7],
        vec![7,1,3],
        vec![1,9,5]
      ]).unwrap()), 17);
  }
}
