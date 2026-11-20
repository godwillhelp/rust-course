# Лекция 1. Cargo, переменные и числа

- Лекция: https://godwillhelp.github.io/rust-course/lessons/01-cargo-variables-numbers/lecture.html (исходник: [lecture.html](lecture.html))
- Книга: гл. 1, 3.1, 3.2
- Rust 1.100.0, edition 2024

## Темы

rustup, rustc, cargo, clippy, rustfmt; каналы и редакции; профили dev и release; как читать ошибки компилятора и линты; `let`, `mut`, `const`, затенение; целые, дробные, `bool`, `char`; литералы; `From` и `as`; кортежи и массивы; переполнение в debug и release, методы `wrapping`/`checked`/`saturating`/`overflowing`; `println!` и форматирование.

## Задание 1. Кошелёк, здоровье и тики (10 баллов, с тестами)

Реализуйте восемь функций в [task1/src/lib.rs](task1/src/lib.rs) так, чтобы прошли тесты [task1/tests/task1.rs](task1/tests/task1.rs):

```
cargo test -p l01-task1
```

Когда все функции готовы, удалите строку `#![allow(unused_variables)]`. Добавьте минимум два своих теста на граничные случаи.

## Задание 2. Анатомия чисел (10 баллов, открытое)

Программа [task2/src/main.rs](task2/src/main.rs) (`cargo run -p l01-task2`) и отчёт [task2/REPORT.md](task2/REPORT.md):

1. **Таблица типов.** Ровная таблица для всех 12 целых типов: имя, размер в байтах (`size_of`), MIN, MAX. Числа выровнены по правому краю.
2. **Четыре стратегии.** Для `200u8 + 100` и `-128i8 - 1` напечатать результаты `wrapping`, `checked`, `saturating`, `overflowing`. Значения пропустить через `std::hint::black_box`.
3. **debug против release.** Простое `+` с переполнением: запустить в обоих профилях, записать в отчёт, что произошло и почему, затем убрать эту строку.
4. **Предсказания.** 6 своих приведений через `as` (не из лекции): сначала предсказание, потом факт и объяснение промахов.
5. **Бестиарий.** Вызвать минимум три разные ошибки компилятора и записать их в [bestiary.md](../../bestiary.md).

Критерии: правильность 4 (таблица и стратегии верны, программа завершается без паники), идиоматичность 2 (`cargo fmt` и `cargo clippy` без замечаний), граничные случаи 2 (128-битные типы, MIN/MAX, знаковые), отчёт и защита 2.

## Практика без оценки

- Rustlings 6.5: `intro1`, `intro2`, `variables1`–`variables6`, `primitive_types1`, `2`, `3`, `5`, `6`.
- Exercism: [Assembly Line](https://exercism.org/tracks/rust/exercises/assembly-line).

## Сдача

`cargo fmt`, `cargo clippy --workspace --all-targets`, `cargo test -p l01-task1` без ошибок → push в `main` → сообщение «сдаю задания лекции 1».
