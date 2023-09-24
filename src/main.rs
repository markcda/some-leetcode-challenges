#![feature(trait_alias)]

mod algo_tasks;
mod cf_tools;
mod structs;
mod tests;
mod tools;
mod utils;

use crate::algo_tasks::task1::simple_stock_span_task;
use crate::algo_tasks::{
  leetcode13::leetcode13_task,
  leetcode234::leetcode234_task,
  leetcode383::leetcode383_task,
  leetcode412::leetcode412_task,
  leetcode876::leetcode876_task,
  leetcode1337::leetcode1337_task,
  leetcode1342::leetcode1342_task,
  leetcode1672::leetcode1672_task,
  information_theory_lab1::{
    discrete_random_value_modeling_example, markov_chain_modeling_example,
    discrete_random_value_modeling,         markov_chain_modeling,
  },
};
use crate::tools::{MResult, read_opt};

fn print_ops() {
  println!("1. Простой алгоритм для задачи о разнице курсов акций");
  println!("2. Римские числа в арабские");
  println!("3. Списки-палиндромы");
  println!("4. Сборка слов из набора букв");
  println!("5. FizzBuzz");
  println!("6. Середина односвязного списка");
  println!("7. k слабейших строк матрицы");
  println!("8. Число шагов, чтобы сделать число нулём");
  println!("9. Максимальное богатство");
  println!("10. Моделирование дискретной случайной величины методом суперпозиции [пример]");
  println!("11. Моделирование цепи Маркова [пример]");
  println!("12. Моделирование дискретной случайной величины методом суперпозиции");
  println!("13. Моделирование цепи Маркова");
}

fn select(opt: u16) -> MResult {
  Ok(match opt {
    1u16 => simple_stock_span_task()?,
    2u16 => leetcode13_task()?,
    3u16 => leetcode234_task()?,
    4u16 => leetcode383_task()?,
    5u16 => leetcode412_task()?,
    6u16 => leetcode876_task()?,
    7u16 => leetcode1337_task()?,
    8u16 => leetcode1342_task()?,
    9u16 => leetcode1672_task()?,
    10u16 => discrete_random_value_modeling_example()?,
    11u16 => markov_chain_modeling_example()?,
    12u16 => discrete_random_value_modeling()?,
    13u16 => markov_chain_modeling()?,
    _ => return Err("Программа завершена."),
  })
}

fn main() -> MResult {
  println!("Добрый день! Что требуется?");
  loop {
    print_ops();
    select(read_opt()?)?;
    println!("\nВыберите задачу или введите 0 для выхода:");
  }
}
