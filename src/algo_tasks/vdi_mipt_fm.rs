use crate::tools::{MResult, read_mul, println};

/// Считает n-ное число Фибоначчи по модулю m.
pub fn fib_mod(n: u128, m: u128) -> u128 {
  if n == 0 { 0 }
  else if n <= 2 { 1 }
  else {
    let mut k = 1u128;
    let mut l = 1u128;
    let mut t: u128;
    for _ in 3u128..(n + 1) {
      t = k;
      k = l;
      l = (t + k) % m;
    }
    l
  }
}

pub fn mipt_problem_fm_fibonacci_with_module() -> MResult {
  let nums: Vec<u128> = read_mul(Some("Введите номер числа Фибоначчи и модуль, по которому хотите его получить:"), None)?;
  if nums.len() != 2 { return Err("Вы ввели не два числа"); }
  let a = nums[0]; let b = nums[1];
  println(&format!("{}-е число Фибоначчи по модулю {} = {}.", a, b, fib_mod(a, b)));
  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_fib_with_module() {
    use crate::algo_tasks::vdi_mipt_fm::fib_mod;
    
    assert_eq!(fib_mod(10, 10), 5);
    assert_eq!(fib_mod(11, 10), 9);
  }
}
