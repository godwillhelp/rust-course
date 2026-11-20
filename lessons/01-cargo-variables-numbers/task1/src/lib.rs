#![allow(unused_variables)]

/// Кладёт добычу в кошелёк. Если сумма не помещается в `u32`,
/// кошелёк остаётся полным: `u32::MAX`.
pub fn add_gold(wallet: u32, loot: u32) -> u32 {
    todo!()
}

/// Наносит урон. Здоровье не опускается ниже нуля.
pub fn apply_damage(hp: u16, damage: u16) -> u16 {
    todo!()
}

/// Покупка: `Some(остаток)`, если золота хватает, иначе `None`.
pub fn try_buy(gold: u32, price: u32) -> Option<u32> {
    todo!()
}

/// Счётчик тиков сервера: после 255 идёт 0.
pub fn next_tick(tick: u8) -> u8 {
    todo!()
}

/// Процент здоровья с округлением вниз.
/// Гарантировано: `max_hp > 0` и `hp <= max_hp`.
pub fn hp_percent(hp: u16, max_hp: u16) -> u8 {
    todo!()
}

/// Номер строчной латинской буквы: 'a' → 0, 'z' → 25.
/// Гарантировано: `c` лежит в диапазоне 'a'..='z'.
pub fn letter_index(c: char) -> u8 {
    todo!()
}

/// Изменение температуры `to - from` без потери значения.
pub fn temperature_delta(from: i8, to: i8) -> i16 {
    todo!()
}

/// Средний урон за удар. При `hits == 0` результат подчиняется
/// правилам IEEE 754: 0/0 = NaN, x/0 = +∞.
pub fn average_damage(total: u32, hits: u32) -> f64 {
    todo!()
}
