use l01_task1::*;

#[test]
fn add_gold_adds() {
    assert_eq!(add_gold(10, 5), 15);
    assert_eq!(add_gold(0, 0), 0);
}

#[test]
fn add_gold_caps_at_max() {
    assert_eq!(add_gold(u32::MAX - 1, 5), u32::MAX);
    assert_eq!(add_gold(u32::MAX, u32::MAX), u32::MAX);
}

#[test]
fn apply_damage_subtracts() {
    assert_eq!(apply_damage(100, 30), 70);
    assert_eq!(apply_damage(100, 100), 0);
}

#[test]
fn apply_damage_never_goes_below_zero() {
    assert_eq!(apply_damage(10, 50), 0);
    assert_eq!(apply_damage(0, u16::MAX), 0);
}

#[test]
fn try_buy_with_enough_gold() {
    assert_eq!(try_buy(100, 30), Some(70));
    assert_eq!(try_buy(30, 30), Some(0));
    assert_eq!(try_buy(0, 0), Some(0));
}

#[test]
fn try_buy_without_enough_gold() {
    assert_eq!(try_buy(29, 30), None);
    assert_eq!(try_buy(0, u32::MAX), None);
}

#[test]
fn next_tick_counts_up() {
    assert_eq!(next_tick(0), 1);
    assert_eq!(next_tick(41), 42);
}

#[test]
fn next_tick_wraps_after_255() {
    assert_eq!(next_tick(254), 255);
    assert_eq!(next_tick(255), 0);
}

#[test]
fn hp_percent_rounds_down() {
    assert_eq!(hp_percent(50, 100), 50);
    assert_eq!(hp_percent(1, 3), 33);
    assert_eq!(hp_percent(2, 3), 66);
    assert_eq!(hp_percent(0, 10), 0);
}

#[test]
fn hp_percent_handles_large_hp() {
    assert_eq!(hp_percent(u16::MAX, u16::MAX), 100);
    assert_eq!(hp_percent(60_000, 65_000), 92);
}

#[test]
fn letter_index_maps_alphabet() {
    assert_eq!(letter_index('a'), 0);
    assert_eq!(letter_index('m'), 12);
    assert_eq!(letter_index('z'), 25);
}

#[test]
fn temperature_delta_small_values() {
    assert_eq!(temperature_delta(10, 25), 15);
    assert_eq!(temperature_delta(25, 10), -15);
    assert_eq!(temperature_delta(-5, -5), 0);
}

#[test]
fn temperature_delta_full_range() {
    assert_eq!(temperature_delta(-128, 127), 255);
    assert_eq!(temperature_delta(127, -128), -255);
}

#[test]
fn average_damage_divides() {
    assert_eq!(average_damage(10, 4), 2.5);
    assert_eq!(average_damage(7, 1), 7.0);
    assert_eq!(average_damage(u32::MAX, 1), 4_294_967_295.0);
}

#[test]
fn average_damage_with_zero_hits() {
    assert!(average_damage(0, 0).is_nan());
    assert_eq!(average_damage(5, 0), f64::INFINITY);
}
