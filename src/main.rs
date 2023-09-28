#![feature(trait_alias)]

mod algo_tasks;
mod cf_tools;
mod structs;
mod tools;
mod utils;

use std::process::exit;

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
  codeforces_mgotu_qualification_a::cf_mgotu_qv_a,
  vdi_mipt_rl::mipt_problem_rl_find_gcd,
  vdi_mipt_cf::mipt_problem_cf_chained_fractions,
  vdi_mipt_fm::mipt_problem_fm_fibonacci_with_module,
  vdi_mipt_pp::mipt_problem_pp_fibonacci_with_pisano_period,
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
  println!("14. Вращайте барабан! (https://codeforces.com/problemset/problem/1804/C)");
  println!("15. Нахождение НОД (бинарный алгоритм Евклида) - задача МФТИ [RL](http://olymp1.vdi.mipt.ru/)");
  println!("16. Разряды цепной дроби - задача МФТИ [CF](http://olymp1.vdi.mipt.ru/)");
  println!("17. Числа Фибоначчи по модулю");
  println!("18. Числа Фибоначчи по модулю с применением периодов Пизано");
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
    14u16 => cf_mgotu_qv_a()?,
    15u16 => mipt_problem_rl_find_gcd()?,
    16u16 => mipt_problem_cf_chained_fractions()?,
    17u16 => mipt_problem_fm_fibonacci_with_module()?,
    18u16 => mipt_problem_pp_fibonacci_with_pisano_period()?,
    _ => exit(0),
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
