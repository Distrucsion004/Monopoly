use rand::Rng;

pub fn Props() -> ([Prop;22], [Railroad; 4], [Utility; 2], [Tax; 2], [Chance; 3], [Chest; 3], [Special; 4]) {
    //let mut r = Vec::new();
    let board: [&str; 40] = ["Go", "Mediteranian Avenue", "Community Chest",
                                "Baltic Avenue", "Income Tax", "Reading Railroad",
                                "Oriental Avenue", "Chance", "Vermont Avenue",
                                "Connecticut Avenue", "Jail", "St. Charles Place",
                                "Electric Company", "States Avenue", "Virginia Avenue",
                                "Pensylavania Railroad", "St. James Place", "Community Chest",
                                "Tenessee Avenue", "New York Avenue", "Free Parking",
                                "Kentucky Avenue", "Chance", "Indian Avenue",
                                "Illinois Avenue", "B & O Railroad", "Atlantic Avenue",
                                "Ventnor Avenue", "Water Works", "Marvin Gardens",
                                "Go to Jail", "Pacific Avenue", "North Carolina Avenue",
                                "Community chest", "Pennsylvania Avenue", "Short Line",
                                "Chance", "Park Place", "Luxury Tax", "BoardWalk"];
    let mut props :[space;22]  = [space::Prop(Prop{name : board[1].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :1 , owned : false }),
                                 space::Prop(Prop{name : board[3].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :3 , owned : false}),
                                 space::Prop(Prop{name : board[6].to_string(),  class : PropTypes::Cyan,     price : 100, houses : 0, hotel : false, boardposition :6 , owned : false}),
                                 space::Prop(Prop{name : board[8].to_string(),  class : PropTypes::Cyan,     price : 100, houses : 0, hotel : false, boardposition :8 , owned : false}),
                                 space::Prop(Prop{name : board[9].to_string(),  class : PropTypes::Cyan,     price : 120, houses : 0, hotel : false, boardposition :9 , owned : false}),
                                 space::Prop(Prop{name : board[11].to_string(), class : PropTypes::Pink,     price : 140, houses : 0, hotel : false, boardposition :11, owned : false}),
                                 space::Prop(Prop{name : board[13].to_string(), class : PropTypes::Pink,     price : 140, houses : 0, hotel : false, boardposition :13, owned : false}),
                                 space::Prop(Prop{name : board[14].to_string(), class : PropTypes::Pink,     price : 160, houses : 0, hotel : false, boardposition :14, owned : false}),
                                 space::Prop(Prop{name : board[16].to_string(), class : PropTypes::Orange,   price : 180, houses : 0, hotel : false, boardposition :16, owned : false}),
                                 space::Prop(Prop{name : board[18].to_string(), class : PropTypes::Orange,   price : 180, houses : 0, hotel : false, boardposition :18, owned : false}),
                                 space::Prop(Prop{name : board[19].to_string(), class : PropTypes::Orange,   price : 200, houses : 0, hotel : false, boardposition :19, owned : false}),
                                 space::Prop(Prop{name : board[21].to_string(), class : PropTypes::Red,      price : 220, houses : 0, hotel : false, boardposition :21, owned : false}),
                                 space::Prop(Prop{name : board[23].to_string(), class : PropTypes::Red,      price : 220, houses : 0, hotel : false, boardposition :23, owned : false}),
                                 space::Prop(Prop{name : board[24].to_string(), class : PropTypes::Red,      price : 240, houses : 0, hotel : false, boardposition :24, owned : false}),
                                 space::Prop(Prop{name : board[26].to_string(), class : PropTypes::Yellow,   price : 260, houses : 0, hotel : false, boardposition :26, owned : false}),
                                 space::Prop(Prop{name : board[27].to_string(), class : PropTypes::Yellow,   price : 260, houses : 0, hotel : false, boardposition :27, owned : false}),
                                 space::Prop(Prop{name : board[29].to_string(), class : PropTypes::Yellow,   price : 280, houses : 0, hotel : false, boardposition :29, owned : false}),
                                 space::Prop(Prop{name : board[31].to_string(), class : PropTypes::Green,    price : 300, houses : 0, hotel : false, boardposition :31, owned : false}),
                                 space::Prop(Prop{name : board[32].to_string(), class : PropTypes::Green,    price : 300, houses : 0, hotel : false, boardposition :32, owned : false}),
                                 space::Prop(Prop{name : board[34].to_string(), class : PropTypes::Green,    price : 320, houses : 0, hotel : false, boardposition :34, owned : false}),
                                 space::Prop(Prop{name : board[37].to_string(), class : PropTypes::Blue,     price : 350, houses : 0, hotel : false, boardposition :37, owned : false}),
                                 space::Prop(Prop{name : board[39].to_string(), class : PropTypes::Blue,     price : 400, houses : 0, hotel : false, boardposition :39, owned : false}),
                                ];
    let railroads :[space; 4] = [space::Railroad(Railroad{name : board[5].to_string(),  price : 200, boardposition : 5,  owned :false}),
                                    space::Railroad(Railroad{name : board[15].to_string(), price : 200, boardposition : 15, owned :false}),
                                    space::Railroad(Railroad{name : board[25].to_string(), price : 200, boardposition : 25, owned :false}),
                                    space::Railroad(Railroad{name : board[35].to_string(), price : 200, boardposition : 35, owned :false})
                                    ];
    let utilities: [space; 2] =   [space::Utility(Utility{name : board[12].to_string(), price :150, boardposition : 12, owned : false}),
                                     space::Utility(Utility{name : board[28].to_string(), price :150, boardposition : 28, owned : false})
                                    ];
    let taxes : [space; 2] = [space::Tax(Tax{name: board[4].to_string(),  value : 200, boardposition : 4}),
                            space::Tax(Tax{name: board[38].to_string(), value : 75,  boardposition : 38})
                            ];
    let chances : [space; 3] = [space::Chance(Chance{ boardposition : 7}),
                                 space::Chance(Chance{ boardposition : 22}),
                                 space::Chance(Chance{ boardposition : 36})];
    let chests : [space; 3] = [ space::Chest(Chest{boardposition : 2}),
                                space::Chest(Chest{boardposition : 17}),
                                space::Chest(Chest{boardposition : 33}),
                                ];
    let specials : [space; 4] = [ space::Special(Special{typ : Specia::Go, boardposition : 0}),
                                    space::Special(Special{typ : Specia::Jail, boardposition : 10 }),
                                    space::Special(Special{typ : Specia::Parking, boardposition : 20}),
                                    space::Special(Special{typ : Specia::GoToJail, boardposition : 30})
    ];
    
    let mut ret: [space; 40];
    for i in 0..23{
        ret[i] = props[i];
    }

    return ret;

}

#[derive(Debug)]
pub struct Railroad{
    pub name : String,
    pub price : i32,
    pub boardposition : i32,
    pub owned : bool
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
    Blue
}
#[derive(Debug)]
pub enum Specia{
    Go,
    Jail,
    GoToJail,
    Parking
}
#[derive(Debug)]
pub struct Prop{
    pub name : String, 
    pub class : PropTypes,
    pub price : u32,
    pub houses : u8,
    pub hotel : bool,
    pub boardposition : i8,
    pub owned : bool
}
#[derive(Debug)]
pub struct Chance{
    pub boardposition : i32
}
#[derive(Debug)]
pub struct Chest{
    pub boardposition : i32
}
#[derive(Debug)]
pub struct Tax{
    pub name : String,
    pub boardposition : i32,
    pub value : i32
}
#[derive(Debug)]
pub struct Utility{
    pub name : String,
    pub price : i32,
    pub boardposition : i32,
    pub owned : bool
}


#[derive(Debug)]
pub struct Special{
    typ : Specia,
    boardposition : i32
}

pub enum space{
    Prop(Prop),
    Railroad (Railroad),
    Utility (Utility),
    Special (Special),
    Tax(Tax),
    Chance (Chance),
    Chest (Chest)
}
/*pub struct space{
    ty : t,
    pos : i32
    
}*/
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
pub struct Player{
    money: i32,
    boardposition: i32,
    railroads: Vec<String>,
    utilities: Vec<String>,
    props: Vec<String>
}