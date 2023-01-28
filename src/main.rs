use player::{Player, initialize_player};
use rand::Rng;
use std::{env };
use crate::{ data::{SpaceType, PropTypes, win_check}};
mod data;
use std::io;
use std::time::{ Instant};

//add invalid input exceptions to buy hotel and buy house funtionallity and add buy in bulk option
//fix weird output of hotelable options
// 
mod player;
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let player_count = &args[1];
    let count = player_count.parse::<i32>().unwrap();
    println!("{}", count);
    let mut playe_r:Vec<Player> = Vec::new();
    for player in 0..count{
        playe_r.push(initialize_player(player as i8 + 1));
    }
    let names = data::props().0;
    let mut spaces = data::props().1;
    let mut dice = data::Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7),
    };
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("Enter \"help\" for a list of commands");
    'master: loop{
    for  i in 0..count{
    println!("Player {}'s turn:", i+1);
    let mut moved = false;
    loop{
        
        if win_check(&playe_r).0 == true{
            println!("The winner is Player{}", win_check(&playe_r).1);
            break 'master
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //this block is used in house buying
        let mut u =0;
        let words: Vec<&str> = input.split_whitespace().collect();
        
        if words.len() == 2 && words[0].trim() == "buy_house"{
            u = words[1].parse().unwrap();
            input = words[0].to_string();
        }
        else {
            u = 1;
        }
        
        
        match input.trim(){
            "help" => list_commands(),
            "wallet" => println!("Balance = {}",playe_r[i as usize].money),
            "roll" => {dice.roll();
                println!("{}, {}", &dice.d1, &dice.d2);},
            "move" => { if moved == false{
                        playe_r[i as usize].dice_move(dice.total());
                        println!("You landed on {}",spaces[playe_r[i as usize].boardposition as usize].name);
                        if spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Special{
                            match &*spaces[playe_r[i as usize].boardposition as usize].name{
                                "Go" => println!("200 $ have been added to your wallet"),
                                _ => println!("OK")
                            }
                        }
                        if spaces[playe_r[i as usize].boardposition as usize].owned == true && spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Prop{
                        let x = match spaces[playe_r[i as usize].boardposition as usize].houses{
                                            0 => spaces[playe_r[i as usize].boardposition as usize].rent.basic,
                                            1 => spaces[playe_r[i as usize].boardposition as usize].rent.house1,
                                            2 => spaces[playe_r[i as usize].boardposition as usize].rent.house2,
                                            3 => spaces[playe_r[i as usize].boardposition as usize].rent.house3,
                                            4 => {if spaces[playe_r[i as usize].boardposition as usize].hotel == false{ 
                                                        spaces[playe_r[i as usize].boardposition as usize].rent.house4
                                                }
                                                else {
                                                    spaces[playe_r[i as usize].boardposition as usize].rent.hotel
                                                }
                                            },
                                            _ => panic!("Problem in getting rent price")
                                    };
                        println!("Owned by player {},\nYou have to pay {} $ in rent", spaces[playe_r[i as usize].boardposition as usize].owner, x);
                        playe_r[i as usize].take_money(x);
                        let y = playe_r[i as usize].boardposition;
                        playe_r[spaces[y as usize].owner as usize -1].add_money(x);
                        println!("New balance = {}", playe_r[i as usize].money);
                            }  
                        else if spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Prop {println!("Price = {}", spaces[playe_r[i as usize].boardposition as usize].price  )}
                        moved = true /*false*/}
                        else {println!("You already moved!")};

                        },
            "check"=> {println!("{}",spaces[playe_r[i as usize].boardposition as usize].name)},
            "buy" =>  {if ( spaces[playe_r[i as usize].boardposition as usize].kind == SpaceType::Prop      ||
                            spaces[playe_r[i as usize].boardposition as usize].kind == SpaceType::Utility   ||
                            spaces[playe_r[i as usize].boardposition as usize].kind == SpaceType::Railroad) && 
                            spaces[playe_r[i as usize].boardposition as usize].owned == false{
                                let k = &mut spaces[playe_r[i as usize].boardposition as usize];
                                playe_r[i as usize].buy_prop(k);
                                println!("Sucessfully bought {} \nNew balance is {}",spaces[playe_r[i as usize].boardposition as usize].name , playe_r[i as usize].money);
                        }
                        else if spaces[playe_r[i as usize].boardposition as usize].owned == true{
                            println!("Space is already owned by player {}",spaces[playe_r[i as usize].boardposition as usize].owner);
                        }
                        else{println!("Not a buyable field");}
               },
            "buy_house" => {println!("Type the name of the property you want to put a house on:\n{:?}", playe_r[i as usize].props);
                        let mut choice = String::new();
                        io::stdin().read_line(&mut choice).unwrap();
                        for _p in 0..u{
                        'exception :for q in 0.. playe_r[i as usize].props.len(){
                            
                            if choice.trim() == playe_r[i as usize].props[q]{
                                for w in 0..40{
                                    let check = &spaces[w];
                                    if choice.trim() == names[w] && playe_r[i as usize].check_set(check, &spaces) == false { 
                                        println!("You don't own the whole set!");
                                        break 'exception;
                                    } 
                                    else if choice.trim() == names[w]{
                                        
                                        if spaces[w].houses >=4{
                                            println!("You already have 4 houses on this field!");
                                        }
                                    
                                        else {spaces[w].houses +=  1;
                                        playe_r[i as usize].take_money(spaces[w].housep);
                                        
                                        println!("Succesfully added house on {}", names[w]);}
                                    }
                                }   
                            }
                        }}
                        }
            "buy_hotel" => {
                            let mut hotelable: Vec<String> = Vec::new();
                            let mut choice = String::new();
                            
                            for q in 0.. playe_r[i as usize].props.len(){
                               
                                    for w in 0..40{
                                        if playe_r[i as usize].props[q] == names[w] && spaces[w].houses == 4 && spaces[w].hotel == false{
                                            hotelable.push(names[w].to_string());
                                        }
                                    
                                }
                            }
                            println!("Type the name of the property you want to put a hotel on:\n{:?}", hotelable);
                            io::stdin().read_line(&mut choice).unwrap();
                            if hotelable.contains(&choice.trim().to_string()){
                            for q in 0.. playe_r[i as usize].props.len(){
                                if choice.trim() == playe_r[i as usize].props[q]{
                                    for w in 0..40{
                                        if choice.trim() == spaces[w].name{
                                            spaces[w].hotel = true;
                                            playe_r[i as usize].take_money(spaces[w].housep);
                                            println!("Succesfully added hotel on {}", choice.trim());
                                        }
                                        }
                                }
                                    
                                }
                            }
                            else {println!("Not a valid field!")}
            }
            
            "owned" => {println!("props = {:?},\n railroads = {:?},\n utilities = {:?}\n", playe_r[i as usize].props, playe_r[i as usize].railroads, playe_r[i as usize].utilities);}
            "end" => {break},
            "QUIT" => {break 'master}
            _ => (),        
    }}
    
}
}

}



fn list_commands()->(){
    print!("
    \"wallet\"    - check how much money you have
    \"roll \"     - rolls dice
    \"move\"      - move as many spaces as the dice show
    \"owned\"     - see owned fields
    \"buy\"       - buy the space you are located on
    \"buy_house\" - add a house to a property
    \"buy_hotel\" - add a hotel to a property
    \"loan\"      - take a loan
    \"trade_prop\"- trade one of your properties
    \"mortgage\"  - get a mortgage on one of your properties
    \"end\"       - end your turn
            " );
}
