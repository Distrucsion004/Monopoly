use player::{Player, initialize_player};
use rand::Rng;
use std::{env, ops::Index, collections::HashMap};
use crate::player::{update_ownership};
mod data;
use data::{Space};
mod player;
fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let player_count = &args[1];
    let count = player_count.parse::<i32>().unwrap();
    println!("{}", count);
    let mut playe_r:Vec<Player> = Vec::new();
    for player in 0..count{
        playe_r.push(initialize_player());
    }*/

    /*
    for i in playe_r{
        println!("{:?}", i);
    }*/
    
    let mut spaces = data::props();
    
    
    let mut dice = data::Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
    };

    //debugging code 
   /* 
    let mut player = player::initialize_player();
    player.add_money(10);
    println!("{}", player.money);
    player.take_money(40);
    println!("{}", player.money);
    player.dice_move(5);
    println!("{}", player.boardposition);
    player.add_railroad(&spaces[5]);
    update_ownership(&mut spaces[5]);
    println!("{:?}", player.railroads);
    println!("{}", spaces[5].owned);
    */

}

fn check_space_type(u : &Player, nam : &[&str; 40], boa: &HashMap<String, String>) -> String{
    let mut r : String;
    r  = (&boa[nam[u.boardposition as usize]]).to_string();
    return r;
}


