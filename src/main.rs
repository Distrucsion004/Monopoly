use player::{Player, initialize_player};
use rand::Rng;
use std::{env, ops::Index, collections::HashMap};
use crate::{player::{update_ownership}, data::space_type};
mod data;
use data::{Space};
use std::io;
mod player;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let player_count = &args[1];
    let count = player_count.parse::<i32>().unwrap();
    println!("{}", count);
    let mut playe_r:Vec<Player> = Vec::new();
    let mut back:Vec<Player> = Vec::new();
    for player in 0..count{
        playe_r.push(initialize_player());
    }
    
    let mut spaces = data::props();
    let mut dice = data::Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7),

    };
    println!("Enter \"help\" for a list of commands");
    loop{
    for mut i in 0..count{
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        //input = input.trim();
        match input.trim(){
            "help" => list_commands(),
            "roll" => {dice.roll();
                println!("{}, {}", &dice.d1, &dice.d2);},
            "move" => { playe_r[i as usize].dice_move(dice.total());
                        println!("You landed on {}",spaces[playe_r[i as usize].boardposition as usize].name)},
            "check"=> {println!("{}",spaces[playe_r[i as usize].boardposition as usize].name)},
            "buy" =>  {if ( spaces[playe_r[i as usize].boardposition as usize].kind == space_type::Prop ||
                            spaces[playe_r[i as usize].boardposition as usize].kind == space_type::Utility ||
                            spaces[playe_r[i as usize].boardposition as usize].kind == space_type::Railroad) && 
                            spaces[playe_r[i as usize].boardposition as usize].owned == false{
                                let k = &mut spaces[playe_r[i as usize].boardposition as usize];
                                playe_r[i as usize].buy_prop(k);
                        }
                        else if spaces[playe_r[i as usize].boardposition as usize].owned == true{
                            println!("Space is already owned!");
                        }
                        else{println!("Not a buyable field");}
               }
            "owned" => {println!("props = {:?},\n railroads = {:?},\n utilities = {:?}\n", playe_r[i as usize].props, playe_r[i as usize].railroads, playe_r[i as usize].utilities);}
            "end_turn" => {break},
            
            _ => (),

        }
        
    }
    
}
}
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
    //debugging code 
    /*
    let mut player = player::initialize_player();
    player.add_money(10);
    println!("{}", player.money);
    player.take_money(40);
    println!("{}", player.money);
    player.dice_move(5);
    println!("{}", player.boardposition);
    
    
    println!("{:?}", player.railroads);
    println!("{}", spaces[5].owned);
    player.buy_prop(&mut spaces[5]);
    println!("{:?}", player );
    println!("{}", spaces[5].owned);
    */

}

fn check_space_type(u : &Player, nam : &[&str; 40], boa: &HashMap<String, String>) -> String{
    let mut r : String;
    r  = (&boa[nam[u.boardposition as usize]]).to_string();
    return r;
}

fn list_commands()->(){
    print!("
                \"check\"     - check position on board
                \"roll \"     - rolls dice
                \"move\"      - move as many spaces as the dice show
                \"owned\"     - see owned fields
                \"buy\"       - buy the space you are located on
                \"buy_house\" - add a house to a property
                \"buy_hotel\" - add a hotel to a property
                \"loan\"      - take a loan
                \"trade_prop\"- trade one of your property
                \"mortgage\"  - get a mortgage on one of your properties
                \"end_turn\"  - end your turn
            " );
}
