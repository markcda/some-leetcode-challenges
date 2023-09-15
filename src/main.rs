mod algo_tasks;
mod structs;
mod tests;
mod tools;

use crate::algo_tasks::task1::simple_stock_span_task;
use crate::algo_tasks::{
  leetcode13::leetcode13_task,
  leetcode234::leetcode234_task,
  leetcode383::leetcode383_task,
  leetcode412::leetcode412_task,
  leetcode876::leetcode876_task,
};
use crate::tools::{MResult, read_opt};

fn main() -> MResult {
  println!("Добрый день! Что требуется?");
  loop {
    println!("1. Простой алгоритм для задачи о разнице курсов акций");
    println!("2. Римские числа в арабские");
    println!("3. Списки-палиндромы");
    println!("4. Сборка слов из набора букв");
    println!("5. FizzBuzz");
    println!("6. Середина односвязного списка");
    match read_opt()? {
      1u8 => simple_stock_span_task()?,
      2u8 => leetcode13_task()?,
      3u8 => leetcode234_task()?,
      4u8 => leetcode383_task()?,
      5u8 => leetcode412_task()?,
      6u8 => leetcode876_task()?,
      _ => break,
    };
    println!("\nВыберите задачу:");
  }
  Ok(())
}
