use crate::cf_tools::read_mul;
use crate::tools::{MResult, read_opt};

pub fn cf_mgotu_qv_a() -> MResult {
  let amount_of_data: u16 = read_opt()?;
  let mut answers: Vec<bool> = Vec::with_capacity(amount_of_data as usize);
  let mut bound: i32;
  'loop1: for _ in 0..amount_of_data {
    let data: Vec<i32> = read_mul()?;
    let n = data[0]; // количество секторов на колесе
    let x = data[1]; // текущий сектор, на который указывает стрелка
    let p = data[2]; // максимальная сила вращения
    if p <= n * 2 { bound = p; }
    else { bound = n * 2; }
    for i in 1..(bound + 1) {
      if (i * (i + 1) / 2 + x) % n == 0 {
        answers.push(true);
        continue 'loop1
      }
    }
    answers.push(false);
  }
  for answer in answers { if answer { println!("Yes"); } else { println!("No"); } }
  Ok(())
}