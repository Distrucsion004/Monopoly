use rand::Rng;

use crate::player::{self, Player};

// this function is used to send all the data for every space in the monopoly board to other modules.
pub fn props() -> ([&'static str; 40],[Space;40])   {
    let board: [&str; 40] = [   "Go", "Mediteranian Avenue", "Chest",
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

    let  spaces :[Space;40] = [ Space{name : board[0].to_string(),  class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 0 ,owned : false, mortgage : false, owner : 0,kind: SpaceType::Special,  rent: Rent { basic:0,  house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[1].to_string(),  class : PropTypes::Cherry,   price :60,   houses : 0, hotel : false, boardposition :1 , owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:2 , house1: 10, house2: 30, house3: 90, house4: 160, hotel: 250, mortgage:30         }, housep : 50},
                                Space{name: "Chest" .to_string(),   class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition :2 ,owned : false , mortgage : false, owner : 0,kind: SpaceType::Chest,    rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[3].to_string(),  class : PropTypes::Cherry,   price :60,   houses : 0, hotel : false, boardposition :3 , owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:4 , house1: 20, house2: 60, house3: 180, house4: 320, hotel: 450, mortgage: 30       }, housep : 50},
                                Space{name:  board[4].to_string(),  class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 4 ,owned : false, mortgage : false, owner : 0,kind: SpaceType::Tax,      rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[5].to_string(),  class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 5, owned :false , mortgage : false, owner : 0,kind: SpaceType::Railroad, rent: Rent { basic: 0, house1: 25, house2: 50, house3: 100, house4: 200, hotel: 0, mortgage: 100        }, housep : 0},
                                Space{name : board[6].to_string(),  class : PropTypes::Cyan,     price :100,  houses : 0, hotel : false, boardposition :6 , owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:6 , house1: 30, house2: 90, house3: 270, house4: 400, hotel: 550, mortgage:50        }, housep : 50},
                                Space{name: "Chance".to_string(),   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 7, owned : false, mortgage : false, owner : 0,kind: SpaceType::Chance,   rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[8].to_string(),  class : PropTypes::Cyan,     price :100,  houses : 0, hotel : false, boardposition :8 , owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:6 , house1: 30, house2: 90, house3: 270, house4: 400, hotel: 550, mortgage: 50       }, housep : 50},
                                Space{name : board[9].to_string(),  class : PropTypes::Cyan,     price :120,  houses : 0, hotel : false, boardposition :9 , owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:8 , house1: 40, house2: 100, house3: 300, house4: 450, hotel: 600, mortgage: 60      }, housep : 50},
                                Space{name : board[10].to_string(), class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 10,owned : false, mortgage : false, owner : 0,kind: SpaceType::Special,  rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[11].to_string(), class : PropTypes::Pink,     price :140,  houses : 0, hotel : false, boardposition :11, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:10 ,house1: 50, house2: 150, house3: 450, house4: 625, hotel: 750, mortgage: 70      }, housep : 100},
                                Space{name : board[12].to_string(), class : PropTypes::None,     price :150,  houses : 0, hotel : false, boardposition : 12,owned : false, mortgage : false, owner : 0,kind: SpaceType::Utility,  rent: Rent { basic:0,  house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[13].to_string(), class : PropTypes::Pink,     price :140,  houses : 0, hotel : false, boardposition :13, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:10 ,house1: 50, house2: 150, house3: 450, house4: 625, hotel: 750, mortgage: 70      }, housep : 100},
                                Space{name : board[14].to_string(), class : PropTypes::Pink,     price :160,  houses : 0, hotel : false, boardposition :14, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:12 ,house1: 60, house2: 180, house3: 500, house4: 700, hotel: 900, mortgage: 80      }, housep : 100},
                                Space{name : board[15].to_string(), class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 15,owned :false , mortgage : false, owner : 0,kind: SpaceType::Railroad, rent: Rent { basic: 0, house1: 25, house2: 50, house3: 100, house4: 200, hotel: 0, mortgage: 100        }, housep : 0},
                                Space{name : board[16].to_string(), class : PropTypes::Orange,   price :180,  houses : 0, hotel : false, boardposition :16, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:14 ,house1: 70, house2: 200, house3: 550, house4: 750, hotel: 950, mortgage: 90      }, housep : 100},
                                Space{name: "Chest" .to_string(),   class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 17,owned : false, mortgage : false, owner : 0,kind: SpaceType::Chest,    rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[18].to_string(), class : PropTypes::Orange,   price :180,  houses : 0, hotel : false, boardposition :18, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:16 ,house1: 70, house2: 200, house3: 550, house4: 750, hotel: 950, mortgage: 90      }, housep : 100},
                                Space{name : board[19].to_string(), class : PropTypes::Orange,   price :200,  houses : 0, hotel : false, boardposition :19, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:18 ,house1: 80, house2: 220, house3: 600, house4: 800, hotel: 1000, mortgage: 100    }, housep : 100},
                                Space{name : board[20].to_string(), class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 20,owned : false, mortgage : false, owner : 0,kind: SpaceType::Special,  rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[21].to_string(), class : PropTypes::Red,      price :220,  houses : 0, hotel : false, boardposition :21, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:18 ,house1: 90, house2: 250, house3: 700, house4: 875, hotel: 1050, mortgage: 110    }, housep : 150},
                                Space{name: "Chance".to_string(),   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 22,owned : false, mortgage : false, owner : 0,kind: SpaceType::Chance,   rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[23].to_string(), class : PropTypes::Red,      price :220,  houses : 0, hotel : false, boardposition :23, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:20 ,house1: 90, house2: 250, house3: 700, house4: 875, hotel: 1050, mortgage:  110   }, housep : 150},
                                Space{name : board[24].to_string(), class : PropTypes::Red,      price :240,  houses : 0, hotel : false, boardposition :24, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:22 ,house1: 100, house2: 300, house3: 750, house4: 925, hotel: 1100, mortgage: 120   }, housep : 150},
                                Space{name : board[25].to_string(), class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 25,owned :false , mortgage : false, owner : 0,kind: SpaceType::Railroad, rent: Rent { basic: 0, house1: 25, house2: 50, house3: 100, house4: 200, hotel: 0, mortgage: 100        }, housep : 0},
                                Space{name : board[26].to_string(), class : PropTypes::Yellow,   price :260,  houses : 0, hotel : false, boardposition :26, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:22 ,house1: 110, house2: 330, house3: 800, house4: 975, hotel: 1150, mortgage: 130   }, housep : 150},
                                Space{name : board[27].to_string(), class : PropTypes::Yellow,   price :260,  houses : 0, hotel : false, boardposition :27, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:24 ,house1: 110, house2: 330, house3: 800, house4: 975, hotel: 1150, mortgage:  130  }, housep : 150},
                                Space{name : board[28].to_string(), class : PropTypes::None,     price :150,  houses : 0, hotel : false, boardposition : 28,owned : false, mortgage : false, owner : 0,kind: SpaceType::Utility,  rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[29].to_string(), class : PropTypes::Yellow,   price :280,  houses : 0, hotel : false, boardposition :29, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:26 ,house1: 120, house2: 360, house3: 850, house4: 1025, hotel: 1200, mortgage:  140 }, housep : 150},
                                Space{name : board[30].to_string(), class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 30,owned : false, mortgage : false, owner : 0,kind: SpaceType::Special,  rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[31].to_string(), class : PropTypes::Green,    price :300,  houses : 0, hotel : false, boardposition :31, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:26 ,house1: 130, house2: 390, house3: 900, house4: 1100, hotel: 1275, mortgage: 150  }, housep : 200},
                                Space{name : board[32].to_string(), class : PropTypes::Green,    price :300,  houses : 0, hotel : false, boardposition :32, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:28 ,house1: 130, house2: 390, house3: 900, house4: 1100, hotel: 1275, mortgage: 150  }, housep : 200},
                                Space{name: "Chest" .to_string(),   class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 33,owned : false, mortgage : false, owner : 0,kind: SpaceType::Chest,    rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[34].to_string(), class : PropTypes::Green,    price :320,  houses : 0, hotel : false, boardposition :34, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:35 ,house1: 150, house2: 450, house3: 1000, house4: 1200, hotel: 1400, mortgage:  160}, housep : 200},
                                Space{name : board[35].to_string(), class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 35,owned :false , mortgage : false, owner : 0,kind: SpaceType::Railroad, rent: Rent { basic: 0, house1: 25, house2: 50, house3: 100, house4: 200, hotel: 0, mortgage: 100        }, housep : 0},
                                Space{name: "Chance".to_string(),   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 36,owned : false, mortgage : false, owner : 0,kind: SpaceType::Chance,   rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[37].to_string(), class : PropTypes::Blue,     price :350,  houses : 0, hotel : false, boardposition :37, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:35 ,house1: 175, house2: 500, house3: 1100, house4: 1300, hotel: 1500, mortgage:  175}, housep : 200},
                                Space{name:  board[38].to_string(), class : PropTypes::None,     price :75,   houses : 0, hotel : false, boardposition : 38,owned : false, mortgage : false, owner : 0,kind: SpaceType::Tax,      rent: Rent { basic:0 , house1: 0 , house2: 0 , house3: 0 , house4: 0 , hotel: 0 , mortgage: 0           }, housep : 0},
                                Space{name : board[39].to_string(), class : PropTypes::Blue,     price :400,  houses : 0, hotel : false, boardposition :39, owned : false, mortgage : false, owner : 0,kind: SpaceType::Prop,     rent: Rent { basic:50 ,house1: 200, house2: 600, house3:1400 , house4: 1700, hotel: 2000, mortgage:  200}, housep : 200},
                                ];
                            
    return (board,spaces);

}

//returns the correct rent price for a certain field corresponding to the number of houses on it 
pub fn find_rent( pos : i32,b: &mut [Space;40], ret: i32) -> i32 {
    
    match ret{
        0 => return b[pos as usize].rent.basic,
        1 => return b[pos as usize].rent.house1,
        2 => return b[pos as usize].rent.house2,
        3 => return b[pos as usize].rent.house3,
        4 => {if b[pos as usize].hotel == false{ 
                    return b[pos as usize].rent.house4
            }
            else {
                return b[pos as usize].rent.hotel
            }
        },
        _ => panic!("Problem in getting rent price")
    };
}

pub fn rent_uti(no : i32, mul: i32) -> i32{
    if no == 1{
        return mul * 4
    }
    return mul * 10 
}
//returns the number to send to the match clause in the above function 
pub fn rent_match(ow:&mut Vec<Player>,pos : i32,b: &mut [Space;40])-> i32{
    if b[pos as usize].kind == SpaceType::Railroad{
        println!("{}",b[pos as usize].owner);
        return ow[b[pos as usize].owner as usize - 1].railroads.len() as i32;
    }
    else if b[pos as usize].kind == SpaceType::Utility{
        let yu  = &ow[b[pos as usize].owner as usize - 1];
        return yu.utilities.len() as i32
    }
    else {
        return b[pos as usize].houses as i32;
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PropTypes{
    Cherry,
    Cyan,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    Blue,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SpaceType{
    Prop,
    Utility,
    Railroad,
    Chance,
    Chest,
    Special,
    Tax
}

#[derive(Debug)]
pub struct Rent{
    pub basic : i32,
    pub house1 : i32,
    pub house2 : i32,
    pub house3 : i32,
    pub house4 : i32,
    pub hotel  : i32,
    pub mortgage : i32
}

#[derive(Debug)]
pub struct Space{
    pub name : String,
    pub kind :  SpaceType,
    pub class : PropTypes,
    pub price : i32,
    pub houses : u8,
    pub hotel : bool,
    pub boardposition : i32,
    pub owned : bool,
    pub mortgage : bool,
    pub owner : i8,
    pub rent : Rent,
    pub housep : i32
}

//struct for Dice with a method to roll them
pub struct Dice{
    pub d1 : i8,
    pub d2 : i8,
}

impl Dice{
    pub fn roll(&mut self) -> (){
        
        //self.d1 = rand::thread_rng().gen_range(1..7);
        //self.d2 = rand::thread_rng().gen_range(1..7);
        self.d1 = 1;
        self.d2 = 0;
        }
    pub fn total(&self) -> i8 {
        let x= &self.d1 + &self.d2;
        
        return x ;
    } 
    }
//checks if somebody won the game
pub fn win_check(play : &Vec<player::Player>) -> (bool, i8){
    let mut end = false;
    let mut best = 0;
    let mut winner:i8 = -1;
    for p in 0..play.len(){
        if play[p].money <= 0 && play[p].props.len()==0 && play[p].railroads.len()==0 && play[p].utilities.len()==0{
            end = true;
            for h in 0..play.len(){
                if h != p{
                    if play[h].money > best{
                        best = play[h].money;
                        winner = play[h].number;
                    }
                }
            }
            return (end, winner);
        }
    }
    return (end, -1);
}

//generates jail instance for when a player goes to jail
pub fn jail_init(p : &player::Player) -> Jail{
    let x = Jail{
        rounds_in_jail : 0,
        roundsFailed : 0,
        diceOp : true,
        player : p.number
    };
    return x;
    } 

//jail struct from when a player goes to jail
pub struct Jail{
    pub rounds_in_jail : i32,
    pub roundsFailed: i32,
    pub diceOp : bool,
    pub player : i8
}


impl Jail {
    pub fn jail_prompt(&self, x : &i8) -> (){
        if *x == 0 {
        println!("You are now in jail. To get out you have 2 options:\n1.Pay $50 to get out.\n2. Roll the dice and get out if you get a double/n
        If you fail 3 rounds in a row, you have to pay $50 to get out.");
        }
        else{
        println!("You are in jail. To get out you have 2 options:\n1. Pay $50 to get out.\n2. Roll the dice and get out if you get a double/n
        If you fail 3 rounds in a row, you have to pay $50 to get out.\nYou have already failed {} rounds", self.roundsFailed); }
    }

    pub fn rollSuccess(&mut self, x:i8, y: i8) -> bool{
        if x == y {
            return true
        }
       
        return false
    }

    //this function is called when the dice roll fails
    pub fn fail(&mut self)-> (){
        self.roundsFailed += 1;
        if self.roundsFailed == 3{
        self.diceOp = false;
        }

    }


}

pub fn parkPrompt() -> (){
    println!("Press 1 to pay 100$ to move anywhere on the board ")
}

pub fn freeParking(s:i32) -> (){
    
}

