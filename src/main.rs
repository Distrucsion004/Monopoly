use rand::Rng;
use std::collections::HashMap;
mod data;
fn main() {
    let board = data::Props();
    let mut dice = data::Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
    };
    //println!("{:?}", board.5[0].boardposition);
    for i in 0..41{
        for j in board{
            for n in j{
            println!("{:?}", n.boardposition);
            }
        }
    }
    //println!("{}", dice.roll().d1);
}


