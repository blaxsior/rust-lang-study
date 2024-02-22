use chapter11::{
    add_number,
    hello::{hello_func, Hello},
    Enemy,
};
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(add_number(2, 2), 4);
}

#[test]
#[should_panic]
fn enemy_should_throw_error_if_minus_damage_input() {
    let mut enemy = Enemy::new(200);
    let wrong_damage = -1;
    enemy.take_damage(wrong_damage);
}
