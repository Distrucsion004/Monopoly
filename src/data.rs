use rand::Rng;
//use std::collections::HashMap;
pub fn props() -> [Space;40]   {
    
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
let mut spaces :[Space;40]  = [ Space{name : board[0].to_string(), kind: space_type::Special,  class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 0 ,owned : false},
                                Space{name : board[1].to_string(), kind: space_type::Prop,     class : PropTypes::Cherry,   price :60,   houses : 0, hotel : false, boardposition :1 , owned : false },
                                Space{name: "Chest" .to_string(),  kind: space_type::Chest,    class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition :2 ,owned : false},
                                Space{name : board[3].to_string(), kind: space_type::Prop,     class : PropTypes::Cherry,   price :60,   houses : 0, hotel : false, boardposition :3 , owned : false},
                                Space{name:  board[4].to_string(), kind: space_type::Tax,      class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 4 ,owned : false},
                                Space{name : board[5].to_string(), kind: space_type::Railroad, class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 5, owned :false},
                                Space{name : board[6].to_string(), kind: space_type::Prop,     class : PropTypes::Cyan,     price :100,  houses : 0, hotel : false, boardposition :6 , owned : false},
                                Space{name: "Chance".to_string(),  kind: space_type::Chance,   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 7, owned : false},
                                Space{name : board[8].to_string(), kind: space_type::Prop,     class : PropTypes::Cyan,     price :100,  houses : 0, hotel : false, boardposition :8 , owned : false},
                                Space{name : board[9].to_string(), kind: space_type::Prop,     class : PropTypes::Cyan,     price :120,  houses : 0, hotel : false, boardposition :9 , owned : false},
                                Space{name : board[10].to_string(),kind: space_type::Special,  class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 10,owned : false},
                                Space{name : board[11].to_string(),kind: space_type::Prop,     class : PropTypes::Pink,     price :140,  houses : 0, hotel : false, boardposition :11, owned : false},
                                Space{name : board[12].to_string(),kind: space_type::Utility,  class : PropTypes::None,     price :150,  houses : 0, hotel : false, boardposition : 12,owned : false},
                                Space{name : board[13].to_string(),kind: space_type::Prop,     class : PropTypes::Pink,     price :140,  houses : 0, hotel : false, boardposition :13, owned : false},
                                Space{name : board[14].to_string(),kind: space_type::Prop,     class : PropTypes::Pink,     price :160,  houses : 0, hotel : false, boardposition :14, owned : false},
                                Space{name : board[15].to_string(),kind: space_type::Railroad, class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 15,owned :false},
                                Space{name : board[16].to_string(),kind: space_type::Prop,     class : PropTypes::Orange,   price :180,  houses : 0, hotel : false, boardposition :16, owned : false},
                                Space{name: "Chest" .to_string(),  kind: space_type::Chest,    class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 17,owned : false},
                                Space{name : board[18].to_string(),kind: space_type::Prop,     class : PropTypes::Orange,   price :180,  houses : 0, hotel : false, boardposition :18, owned : false},
                                Space{name : board[19].to_string(),kind: space_type::Prop,     class : PropTypes::Orange,   price :200,  houses : 0, hotel : false, boardposition :19, owned : false},
                                Space{name : board[20].to_string(),kind: space_type::Special,  class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 20,owned : false},
                                Space{name : board[21].to_string(),kind: space_type::Prop,     class : PropTypes::Red,      price :220,  houses : 0, hotel : false, boardposition :21, owned : false},
                                Space{name: "Chance".to_string(),  kind: space_type::Chance,   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 22,owned : false},
                                Space{name : board[23].to_string(),kind: space_type::Prop,     class : PropTypes::Red,      price :220,  houses : 0, hotel : false, boardposition :23, owned : false},
                                Space{name : board[24].to_string(),kind: space_type::Prop,     class : PropTypes::Red,      price :240,  houses : 0, hotel : false, boardposition :24, owned : false},
                                Space{name : board[25].to_string(),kind: space_type::Railroad, class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 25,owned :false},
                                Space{name : board[26].to_string(),kind: space_type::Prop,     class : PropTypes::Yellow,   price :260,  houses : 0, hotel : false, boardposition :26, owned : false},
                                Space{name : board[27].to_string(),kind: space_type::Prop,     class : PropTypes::Yellow,   price :260,  houses : 0, hotel : false, boardposition :27, owned : false},
                                Space{name : board[28].to_string(),kind: space_type::Utility,  class : PropTypes::None,     price :150,  houses : 0, hotel : false, boardposition : 28,owned : false},
                                Space{name : board[29].to_string(),kind: space_type::Prop,     class : PropTypes::Yellow,   price :280,  houses : 0, hotel : false, boardposition :29, owned : false},
                                Space{name : board[30].to_string(),kind: space_type::Special,  class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 30,owned : false},
                                Space{name : board[31].to_string(),kind: space_type::Prop,     class : PropTypes::Green,    price :300,  houses : 0, hotel : false, boardposition :31, owned : false},
                                Space{name : board[32].to_string(),kind: space_type::Prop,     class : PropTypes::Green,    price :300,  houses : 0, hotel : false, boardposition :32, owned : false},
                                Space{name: "Chest" .to_string(),  kind: space_type::Chest,    class: PropTypes::None,      price: 0,    houses : 0, hotel : false, boardposition : 33,owned : false},
                                Space{name : board[34].to_string(),kind: space_type::Prop,     class : PropTypes::Green,    price :320,  houses : 0, hotel : false, boardposition :34, owned : false},
                                Space{name : board[35].to_string(),kind: space_type::Railroad, class : PropTypes::None,     price :200,  houses : 0, hotel : false, boardposition : 35,owned :false},
                                Space{name: "Chance".to_string(),  kind: space_type::Chance,   class : PropTypes::None,     price: 0,    houses : 0, hotel : false, boardposition : 36,owned : false},
                                Space{name : board[37].to_string(),kind: space_type::Prop,     class : PropTypes::Blue,     price :350,  houses : 0, hotel : false, boardposition :37, owned : false},
                                Space{name:  board[38].to_string(),kind: space_type::Tax,      class : PropTypes::None,     price :75,   houses : 0, hotel : false, boardposition : 38,owned : false},
                                Space{name : board[39].to_string(),kind: space_type::Prop,     class : PropTypes::Blue,     price :400,  houses : 0, hotel : false, boardposition :39, owned : false},


                                ];
                                
    
    
    return spaces;

}

pub trait t{
    fn nu()-> (){
        
    }
}
#[derive(Debug)]
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
pub enum space_type{
    Prop,
    Utility,
    Railroad,
    Chance,
    Chest,
    Special,
    Tax
}
#[derive(Debug)]
pub enum Specia{
    Go,
    Jail,
    GoToJail,
    Parking
}


#[derive(Debug)]
pub struct Space{
    pub name : String,
    pub kind :  space_type,
    pub class : PropTypes,
    pub price : u32,
    pub houses : u8,
    pub hotel : bool,
    pub boardposition : i32,
    pub owned : bool
}
impl t for Space{}

pub struct Dice{
    pub d1 : i8,
    pub d2 : i8,
}

impl Dice{
    pub fn roll(mut self) -> Dice{
        self = Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
        };
        return self
    }
}
