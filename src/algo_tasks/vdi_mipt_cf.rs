use crate::tools::{MResult, TResult, read_mul, println};

pub fn find_chained_fraction_numbers(mut a: i32, mut b: i32) -> TResult<Vec<i32>> {
  let mut nums = vec![];
  loop {
    if a < b { core::mem::swap(&mut a, &mut b); }
    if b == 0 { break }
    match a.checked_div(b) {
      Some(res) => nums.push(res),
      None => return Err("Не удалось разделить два числа"),
    }
    a %= b;
  }
  Ok(nums)
}

pub fn mipt_problem_cf_chained_fractions() -> MResult {
  let nums: Vec<i32> = read_mul(Some("Введите два числа:"), None)?;
  if nums.len() != 2 { return Err("Вы ввели не два числа"); }
  let a = nums[0]; let b = nums[1];
  println(&format!("Результат: {:?}", find_chained_fraction_numbers(a, b)?));
  Ok(())
}
