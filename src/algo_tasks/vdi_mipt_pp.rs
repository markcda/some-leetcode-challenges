use crate::tools::{MResult, read_mul, println};

/// Период Пизано.
/// Ищет не последовательность `0, 1`, а предшествующую ей последовательность `1, 0`.
pub fn get_pisano_period(m: u128) -> u128 {
  let mut n = 2u128;
  let mut tmp: u128;
  let mut k = 1u128;
  let mut l = 1u128;
  loop {
    tmp = k;
    k = l;
    l = (tmp + k) % m;
    n += 1;
    if k == 1 && l == 0 { return n }
  }
}

/// Считает n-ное число Фибоначчи по модулю m.
pub fn fib_mod(n: u128, m: u128) -> u128 {
  if n == 0 { 0 }
  else if n <= 2 { 1 }
  else {
    let n = n % get_pisano_period(m);
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

pub fn mipt_problem_pp_fibonacci_with_pisano_period() -> MResult {
  let nums: Vec<u128> = read_mul(Some("Введите номер числа Фибоначчи и модуль, по которому хотите его получить:"), None)?;
  if nums.len() != 2 { return Err("Вы ввели не два числа"); }
  let a = nums[0]; let b = nums[1];
  println(&format!("{}-е число Фибоначчи по модулю {} = {}.", a, b, fib_mod(a, b)));
  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_fib_with_module_by_pisano_period() {
    use crate::algo_tasks::vdi_mipt_pp::fib_mod;
    
    assert_eq!(fib_mod(10, 10), 5);
    assert_eq!(fib_mod(11, 10), 9);
    assert_eq!(fib_mod(2816213588, 30524), 10249);
  }
}
