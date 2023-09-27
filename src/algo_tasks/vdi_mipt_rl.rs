use crate::tools::{MResult, read_mul, println};

/// Имплементация из `uutils`: https://github.com/uutils/coreutils/blob/1eabda91cf35ec45c78cb95c77d5463607daed65/src/uu/factor/src/numeric/gcd.rs
/// Бинарный алгоритм Евклида для нахождения наибольшего общего делителя.
pub fn gcd(u: i32, v: i32) -> i32 {
  let mut v = v.wrapping_abs() as u32;
  if u == 0 { return v as i32; }
  let mut u = u.wrapping_abs() as u32;
  if v == 0 { return u as i32; }
  let gcd_exponent_on_two = (u | v).trailing_zeros();
  u >>= u.trailing_zeros();
  v >>= v.trailing_zeros();
  
  while u != v {
    if u < v { core::mem::swap(&mut u, &mut v); }
    u -= v;
    u >>= u.trailing_zeros();
  }
  
  (u << gcd_exponent_on_two) as i32
}

pub fn mipt_problem_rl_find_gcd() -> MResult {
  let nums: Vec<i32> = read_mul(Some("Введите два числа:"), None)?;
  if nums.len() != 2 { return Err("Вы ввели не два числа"); }
  let a = nums[0]; let b = nums[1];
  println(&format!("Наибольший общий делитель - {}", gcd(a, b)));
  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_binary_gcd() {
    use crate::algo_tasks::vdi_mipt_rl::gcd;
    
    assert_eq!(gcd(14, 8), 2);
  }
}
