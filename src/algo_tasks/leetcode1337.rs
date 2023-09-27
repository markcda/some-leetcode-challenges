use crate::structs::matrix::Matrix;
use crate::tools::{MResult, println, read, read_matrix};

#[derive(PartialEq)]
enum CmpRows {
  FirstWeaker,
  SecondWeaker,
}

/// Функция сравнивает два ряда по их условной "силе".
/// Сила заключается в единичках. Если число единиц совпадает, то мы смотрим на числа `r1i` и `r2i`.
fn cmp_rows(r1: &Vec<i32>, r1i: &i32, r2: &Vec<i32>, r2i: &i32) -> CmpRows {
  let c1 = r1.iter().filter(|&n| *n == 1).count();
  let c2 = r2.iter().filter(|&n| *n == 1).count();
  if c1 < c2        { CmpRows::FirstWeaker }
  else if c2 < c1   { CmpRows::SecondWeaker }
  else if r1i < r2i { CmpRows::FirstWeaker }
  else              { CmpRows::SecondWeaker }
}

pub fn k_weakest_rows(m: Matrix<i32>, k: i32) -> Vec<i32> {
  let ku = k as usize;
  let mut vec = Vec::with_capacity(ku);
  vec.push(0i32);
  for i in 1..m.len() {
    let mut inserted: bool = false;
    for (n, j) in vec.iter().enumerate() {
      if cmp_rows(&m[i], &(i as i32), &m[*j as usize], &j) == CmpRows::FirstWeaker {
        if vec.len() == ku { vec.pop(); }
        vec.insert(n, i as i32);
        inserted = true;
        break
      }
    }
    if !inserted && vec.len() < ku { vec.push(i as i32); }
  }
  vec
}

pub fn leetcode1337_task() -> MResult {
  println("Введите целочисленную матрицу, после ввода пропустите одну строку:");
  let m = read_matrix()?;
  let k = read(Some("Введите число слабейших строк, которые нужно вывести:"))?;
  if k as usize > m.len() { return Err("Число слабейших строк больше числа строк в введённой матрице") }
  let weakest_rows = k_weakest_rows(m, k);
  println(&format!("Ответ: {:?}", weakest_rows));
  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_k_weakest_rows() {
    use crate::structs::matrix::Matrix;
    use crate::algo_tasks::leetcode1337::k_weakest_rows;
    
    assert_eq!(k_weakest_rows(
      Matrix::<i32>::from(
        vec![vec![1,1,0,0,0],
             vec![1,1,1,1,0],
             vec![1,0,0,0,0],
             vec![1,1,0,0,0],
             vec![1,1,1,1,1]],
      ).unwrap(), 3
    ),
    vec![2,0,3]);
    assert_eq!(k_weakest_rows(
      Matrix::<i32>::from(
        vec![vec![1,0,0,0],
             vec![1,1,1,1],
             vec![1,0,0,0],
             vec![1,0,0,0]],
      ).unwrap(), 2
    ),
    vec![0,2]);
  }
}
