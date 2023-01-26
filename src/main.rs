use std::borrow::Borrow;

use rand::Rng;

use crate::player::update_railroads;
mod data;
mod player;
fn main() {
    let things = data::props();
    // stores name - type pairs
    let board = things.1;
    //stores indexed names;
    let names = things.0;
    //store arrays of types
    let mut props = things.2;
    let mut railroads = things.3;
    let mut utilities = things.4;
    let mut taxes = things.5;
    let mut chances = things.6;
    let mut chests = things.7;
    let mut specials = things.8;
    let mut dice = data::Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
    };
    let mut player = player::initialize_player();
    player.add_money(10);
    println!("{}", player.money);
    player.take_money(40);
    println!("{}", player.money);
    player.dice_move(2);
    println!("{}", player.boardposition);
    player.add_railroad(names[railroads[0].boardposition as usize].to_string());
    update_railroads(&mut railroads[0]);
    println!("{:?}", player.railroads);
    println!("{}", railroads[0].owned);

}
    //println!("{}", dice.roll().d1);
//}

/*fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}*/
