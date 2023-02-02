use player::{Player, initialize_player};
use rand::{Rng, seq::index::IndexVecIntoIter};
use std::{env, net::ToSocketAddrs };
use crate::{ data::{SpaceType, PropTypes, win_check, Jail, jail_init, find_rent, rent_match}, chances::{chances, ch1, ch0, ch2, ch3, ch4}};
mod data;
mod chances;

use std::io;
use std::time::{Instant};


mod player;
//fix weird go chance bullshit
//really gotta add comments and clean up many parts of this
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let player_count = &args[1];
    let count = player_count.parse::<i32>().unwrap();
    println!("{}", count);
    let mut playe_r:Vec<Player> = Vec::new();
    let mut jails:Vec<Jail> = Vec::new();
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
    'play: for  i in 0..count{
    println!("Player {}'s turn:", i+1);
    
    for w in 0..40{
        if spaces[w].mortgage == true && playe_r[i as usize].mortgaged.contains_key(&spaces[w].name) == true{
            *playe_r[i as usize].mortgaged.get_mut(&spaces[w].name).unwrap() += 1;
        }
    }
    let mut moved = false;
    'b: loop{
        
        if win_check(&playe_r).0 == true{
            println!("The winner is Player{}", win_check(&playe_r).1);
            break 'master
        }
        
        if playe_r[i as usize].in_jail == true{
            for k in 0..jails.len(){
            if &jails[k].player == &playe_r[i as usize].number{
                loop{
                    if jails[k].rounds_in_jail == 0{
                    jails[k].jail_prompt(&playe_r[i as usize].number);
                }
                    else {println!("IN JAIL")}
                let mut yo = String::new();
                io::stdin().read_line(&mut yo).unwrap();
                yo = yo.to_string();
                match yo.trim(){
                    "roll" =>{
                        if jails[k].diceOp == true{
                        dice.roll();
                        println!("{}, {}", &dice.d1, &dice.d2);
                        
                        if jails[k].rollSuccess(dice.d1, dice.d2) == true{
                            println!("You rolled a double, you are now out of jail!");
                            playe_r[i as usize].leave_jail();
                            break
                        }
                        else{jails[k].fail();
                            println!("You failed. You can try again for {} more rounds", 3 - jails[k].roundsFailed);
                            jails[k].rounds_in_jail += 1;
                        break 'b
                    }
                        ;}
                        
                    },
                    "pay"  =>{playe_r[i as usize].payToLeave();
                        println!("You paid $50 to get out of jail!\nNew balance is {}", playe_r[i as usize].money); 
                        break },
                    "pass" =>{
                        jails[k].rounds_in_jail += 1;
                        continue 'master
                    }
                    
                    &_ => println!("Enter a valid option!"),
                }
            }
            }}
            
            
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
                        if spaces[playe_r[i as usize].boardposition as usize].name == "Chance".to_string(){
                            let ch = rand::thread_rng().gen_range(0..4);
                            let chanc = chances();
                            let act = chanc[ch as usize - 1];
                            println!("You drew: {}", act);
                            match ch {
                                1 => ch0(&mut playe_r[i as usize]),
                                2 => ch1(&mut playe_r[i as usize]), 
                                3 => ch2(&mut playe_r[i as usize]),
                                4 => {ch3(&mut playe_r, i as usize, &mut spaces);
                                    println!("Succesfully moved to {}", names[playe_r[i as usize].boardposition as usize])}
                                5 => {ch4(&mut playe_r, i as usize, &mut spaces);
                                    println!("Succesfully moved to {}", names[playe_r[i as usize].boardposition as usize])}
                                _ => panic!("Chance problem")

                            }

                        }
                        else if spaces[playe_r[i as usize].boardposition as usize].kind == SpaceType::Tax{
                            let pays = spaces[playe_r[i as usize].boardposition as usize].price;
                            playe_r[i as usize].take_money(pays);
                            println!("You have to pay ${}",spaces[playe_r[i as usize].boardposition as usize].price );
                            println!("New balance {}", playe_r[i as usize].money);
                            
                        }
                        else if spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Special{
                            match &*spaces[playe_r[i as usize].boardposition as usize].name{
                                "Go" => println!("200 $ have been added to your wallet"),
                                "Go to Jail" => {jails.push(jail_init(&playe_r[i as usize]));
                                                
                                                playe_r[i as usize].boardposition = 10;
                                                playe_r[i as usize].go_to_jail();
                                                continue
                                            }
                                "Free Parking" => {
                                    println!("Press 1 to pay 100$ to move anywhere on the board\nPress enter to decline ");
                                    let mut mo = String::new();
                                    io::stdin().read_line(&mut mo).unwrap();
                                    
                                    match mo.trim(){
                                        "1" => {
                                            'out: loop{
                                            println!("Enter the name of the field you want to go to (besides chance or chest): ");
                                            let mut mo1 = String::new();
                                            io::stdin().read_line(&mut mo1).unwrap();
                                            for lo in 0..40{
                                                if spaces[lo].name == mo1.trim().to_string(){
                                                    playe_r[i as usize].dice_move(
                                                        if spaces[lo].boardposition > 20{
                                                        spaces[lo].boardposition as i8 - 20
                                                        }
                                                        else{
                                                        20 + spaces[lo].boardposition as i8});
                                                    println!("Succesfully landed on {}", names[playe_r[i as usize].boardposition as usize]);
                                                    if playe_r[i as usize].boardposition == 0{
                                                        println!("200 $ have been added to your wallet");
                                                    }
                                                    playe_r[i as usize].take_money(100);
                                                break 'out

                                                }
                                            }
                                                println!("Not a valid field");}
                                        },
                                        _ => break
                                    }
                                    }
                                
                                
                                _ => println!("OK")
                            }
                            
                        }
                        else if spaces[playe_r[i as usize].boardposition as usize].owned == true && spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Prop{
                        let opi = playe_r[i as usize].boardposition as i32;
                        let ren = rent_match(&mut playe_r, opi, &mut spaces);
                        let x = find_rent( opi, &mut spaces, ren );
                            /*let x = match spaces[playe_r[i as usize].boardposition as usize].houses{
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
                                    };*/
                        println!("Owned by player {},\nYou have to pay {} $ in rent", spaces[playe_r[i as usize].boardposition as usize].owner, x);
                        playe_r[i as usize].take_money(x);
                        let y = playe_r[i as usize].boardposition;
                        playe_r[spaces[y as usize].owner as usize -1].add_money(x);
                        println!("New balance  n= {}", playe_r[i as usize].money);
                            }  
                        else if spaces[playe_r[i as usize].boardposition as usize].kind ==  SpaceType::Prop {println!("Price = {}", spaces[playe_r[i as usize].boardposition as usize].price  )}
                        moved = /*true*/ false;}
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
                        }
                    }
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
            "mortgage" => {
                        let mut mortgageable: Vec<String> = Vec::new();
                            let mut choice = String::new();
                            for q in 0.. playe_r[i as usize].props.len(){
                                for w in 0..40{
                                if playe_r[i as usize].props[q] == names[w] && spaces[w].mortgage == false{
                                mortgageable.push(names[w].to_string());
                                }
                            }
                        }
                        println!("Type the name of the property you want to mortgage:\n{:?}", mortgageable);
                        io::stdin().read_line(&mut choice).unwrap();
                        if mortgageable.contains(&choice.trim().to_string()){
                        for q in 0.. playe_r[i as usize].props.len(){
                            if choice.trim() == playe_r[i as usize].props[q]{
                            for w in 0..40 {
                                if choice.trim() == spaces[w].name{
                                    spaces[w].mortgage = true;
                                    if spaces[w].houses > 0{
                                    println!{"Removed all {} houses from the property.", spaces[w].houses};
                                    let back = spaces[w].houses as i32 * spaces[w].housep /2;
                                    println!("Added ${} to your wallet!", back);
                                    playe_r[i as usize].add_money(back);
                                    }
                                    spaces[w].houses = 0;
                                    playe_r[i as usize].props.remove(q);
                                    playe_r[i as usize].mortgaged.insert(names[w].to_string(), 0);
                                    //let added = playe_r[i as usize].mortgaged.len() - 1;
                                    println!("Succesfully took a mortgage on {} for ${}", names[w].to_string(),spaces[w].rent.mortgage);
                                    playe_r[i as usize ].add_money(spaces[w].rent.mortgage);
                                }
                            }
                        }
                    }
                    }
                    }
            "lift_mortgage" =>{
                                println!("Choose the property you want to lift the mortgage off of:");
                                let mut pay:i32 = 0;
                                println!("{:?}", playe_r[i as usize].mortgaged.keys());
                                let mut choice = String::new();
                                std::io::stdin().read_line(&mut choice).unwrap();
                                choice = choice.trim().to_string();
                                if playe_r[i as usize].mortgaged.contains_key(&choice){
                                    for k in 0..40{
                                        if spaces[k].name == choice{
                                            pay = (spaces[k].rent.mortgage * (100 + (playe_r[i as usize].mortgaged[&choice] as i32 * 10)))/100;
                                            println!("{}", pay);
                                        }
                                    }
                                    pay = playe_r[i as usize].mortgaged[&choice] as i32;
                                }
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
