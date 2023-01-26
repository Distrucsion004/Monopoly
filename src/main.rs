use player::{Player, initialize_player};
use rand::Rng;
use std::{env, ops::Index, collections::HashMap};
use crate::player::update_railroads;
mod data;
use data::{Prop, PropTypes, Railroad, Utility, Tax, Chance, Chest, Special};
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

    //debugging code 
   /*
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
    let o =check_space_type(&player, &names, &board);
    println!("{:?}", o);
    */

}
fn check_space_type(u : &Player, nam : &[&str; 40], boa: &HashMap<String, String>) -> String{
    let mut r : String;
    r  = (&boa[nam[u.boardposition as usize]]).to_string();
    return r;
}
fn check_ownership<I:data::t>(th : I) -> bool{
    let board: [&str; 40] = ["Go", "Mediteranian Avenue", "Chest",
                                "Baltic Avenue", "Income Tax", "Reading Railroad",
                                "Oriental Avenue", "Chance", "Vermont Avenue",
                                "Connecticut Avenue", "Jail", "St. Charles Place",
                                "Electric Company", "States Avenue", "Virginia Avenue",
                                "Pensylavania Railroad", "St. James Place", "Chest",
                                "Tenessee Avenue", "New York Avenue", "Free Parking",
                                "Kentucky Avenue", "Chance", "Indian Avenue",
                                "Illinois Avenue", "B & O Railroad", "Atlantic Avenue",
                                "Ventnor Avenue", "Water Works", "Marvin Gardens",
                                "Go to Jail", "Pacific Avenue", "North Carolina Avenue",
                                "Chest", "Pennsylvania Avenue", "Short Line",
                                "Chance", "Park Place", "Luxury Tax", "BoardWalk"];
    let  props :[Prop;22]  = [Prop{name : board[1].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :1 , owned : false },
                                 Prop{name : board[3].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :3 , owned : false},
                                 Prop{name : board[6].to_string(),  class : PropTypes::Cyan,     price : 100, houses : 0, hotel : false, boardposition :6 , owned : false},
                                 Prop{name : board[8].to_string(),  class : PropTypes::Cyan,     price : 100, houses : 0, hotel : false, boardposition :8 , owned : false},
                                 Prop{name : board[9].to_string(),  class : PropTypes::Cyan,     price : 120, houses : 0, hotel : false, boardposition :9 , owned : false},
                                 Prop{name : board[11].to_string(), class : PropTypes::Pink,     price : 140, houses : 0, hotel : false, boardposition :11, owned : false},
                                 Prop{name : board[13].to_string(), class : PropTypes::Pink,     price : 140, houses : 0, hotel : false, boardposition :13, owned : false},
                                 Prop{name : board[14].to_string(), class : PropTypes::Pink,     price : 160, houses : 0, hotel : false, boardposition :14, owned : false},
                                 Prop{name : board[16].to_string(), class : PropTypes::Orange,   price : 180, houses : 0, hotel : false, boardposition :16, owned : false},
                                 Prop{name : board[18].to_string(), class : PropTypes::Orange,   price : 180, houses : 0, hotel : false, boardposition :18, owned : false},
                                 Prop{name : board[19].to_string(), class : PropTypes::Orange,   price : 200, houses : 0, hotel : false, boardposition :19, owned : false},
                                 Prop{name : board[21].to_string(), class : PropTypes::Red,      price : 220, houses : 0, hotel : false, boardposition :21, owned : false},
                                 Prop{name : board[23].to_string(), class : PropTypes::Red,      price : 220, houses : 0, hotel : false, boardposition :23, owned : false},
                                 Prop{name : board[24].to_string(), class : PropTypes::Red,      price : 240, houses : 0, hotel : false, boardposition :24, owned : false},
                                 Prop{name : board[26].to_string(), class : PropTypes::Yellow,   price : 260, houses : 0, hotel : false, boardposition :26, owned : false},
                                 Prop{name : board[27].to_string(), class : PropTypes::Yellow,   price : 260, houses : 0, hotel : false, boardposition :27, owned : false},
                                 Prop{name : board[29].to_string(), class : PropTypes::Yellow,   price : 280, houses : 0, hotel : false, boardposition :29, owned : false},
                                 Prop{name : board[31].to_string(), class : PropTypes::Green,    price : 300, houses : 0, hotel : false, boardposition :31, owned : false},
                                 Prop{name : board[32].to_string(), class : PropTypes::Green,    price : 300, houses : 0, hotel : false, boardposition :32, owned : false},
                                 Prop{name : board[34].to_string(), class : PropTypes::Green,    price : 320, houses : 0, hotel : false, boardposition :34, owned : false},
                                 Prop{name : board[37].to_string(), class : PropTypes::Blue,     price : 350, houses : 0, hotel : false, boardposition :37, owned : false},
                                 Prop{name : board[39].to_string(), class : PropTypes::Blue,     price : 400, houses : 0, hotel : false, boardposition :39, owned : false},
                                ];
    let railroads :[Railroad; 4] = [Railroad{name : board[5].to_string(),  price : 200, boardposition : 5,  owned :false},
                                Railroad{name : board[15].to_string(), price : 200, boardposition : 15, owned :false},
                                Railroad{name : board[25].to_string(), price : 200, boardposition : 25, owned :false},
                                Railroad{name : board[35].to_string(), price : 200, boardposition : 35, owned :false}
                                ];
    let utilities: [Utility; 2] =   [ Utility{name : board[12].to_string(), price :150, boardposition : 12, owned : false},
                                    Utility{name : board[28].to_string(), price :150, boardposition : 28, owned : false}
                                    ];
    let taxes : [Tax; 2] = [Tax{name: board[4].to_string(),  value : 200, boardposition : 4},
                            Tax{name: board[38].to_string(), value : 75,  boardposition : 38}
                            ];
    let chances : [Chance; 3] = [Chance{ boardposition : 7},
                                 Chance{ boardposition : 22},
                                 Chance{ boardposition : 36}];
    let chests : [Chest; 3] = [ Chest{boardposition : 2},
                                Chest{boardposition : 17},
                                Chest{boardposition : 33},
                                ];
    let specials : [Special; 4] = [ Special{name : board[0].to_string(), boardposition : 0},
                                    Special{name : board[10].to_string(), boardposition : 10 },
                                    Special{name : board[20].to_string(), boardposition : 20},
                                    Special{name : board[30].to_string(), boardposition : 30}
                                    ];
    
    
}
