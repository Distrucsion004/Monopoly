use rand::Rng;
use std::collections::HashMap;
pub fn props() -> ([&'static str;40],HashMap<String, String>, [Prop;22], [Railroad; 4], [Utility; 2], [Tax; 2], [Chance; 3], [Chest; 3], [Special; 4] ) {
    let mut r = HashMap::new();
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
    let mut props :[Prop;22]  = [Prop{name : board[1].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :1 , owned : false },
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
    
    for j in props{
        r.insert(j.name,
            "Prop".to_string());

    }
    for j in railroads{
        r.insert(j.name,
            "Ralroad".to_string());

    }
    for j in chances{
        r.insert("Chance".to_string(),
        "Chance".to_string()
        );
    }
    for j in chests{
        r.insert("Chest".to_string(),
        "Chest".to_string()
        );
    }
    for j in specials{
        r.insert(j.name,
            "Special".to_string());

    }
    for j in taxes{
        r.insert(j.name,
            "Tax".to_string());

    }
    for j in utilities{
        r.insert(j.name,
            "Utility".to_string());

    }
    let mut props :[Prop;22]  = [Prop{name : board[1].to_string(),  class : PropTypes::Cherry,   price : 60,  houses : 0, hotel : false, boardposition :1 , owned : false },
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
    let mut railroads :[Railroad; 4] = [Railroad{name : board[5].to_string(),  price : 200, boardposition : 5,  owned :false},
                                Railroad{name : board[15].to_string(), price : 200, boardposition : 15, owned :false},
                                Railroad{name : board[25].to_string(), price : 200, boardposition : 25, owned :false},
                                Railroad{name : board[35].to_string(), price : 200, boardposition : 35, owned :false}
                                ];
    let mut utilities: [Utility; 2] =   [ Utility{name : board[12].to_string(), price :150, boardposition : 12, owned : false},
                                    Utility{name : board[28].to_string(), price :150, boardposition : 28, owned : false}
                                    ];
    let mut taxes : [Tax; 2] = [Tax{name: board[4].to_string(),  value : 200, boardposition : 4},
                            Tax{name: board[38].to_string(), value : 75,  boardposition : 38}
                            ];
    let mut chances : [Chance; 3] = [Chance{ boardposition : 7},
                                 Chance{ boardposition : 22},
                                 Chance{ boardposition : 36}];
    let mut chests : [Chest; 3] = [ Chest{boardposition : 2},
                                Chest{boardposition : 17},
                                Chest{boardposition : 33},
                                ];
    let mut specials : [Special; 4] = [ Special{name : board[0].to_string(), boardposition : 0},
                                    Special{name : board[10].to_string(), boardposition : 10 },
                                    Special{name : board[20].to_string(), boardposition : 20},
                                    Special{name : board[30].to_string(), boardposition : 30}
                                    ];
    return (board,r, props, railroads, utilities, taxes, chances, chests, specials);

}

pub trait t{
    fn nu()-> (){
        
    }
}

#[derive(Debug)]
pub struct Railroad{
    pub name : String,
    pub price : i32,
    pub boardposition : i32,
    pub owned : bool
}
impl t for Railroad{}
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
    pub boardposition : i32,
    pub owned : bool
}
impl t for Prop{}
#[derive(Debug)]
pub struct Chance{
    pub boardposition : i32
}
//impl t for Chance{}
#[derive(Debug)]
pub struct Chest{
    pub boardposition : i32,
}
//impl t for Chest{}
#[derive(Debug)]
pub struct Tax{
    pub name : String,
    pub boardposition : i32,
    pub value : i32
}
//impl t for Tax{}
#[derive(Debug)]
pub struct Utility{
    pub name : String,
    pub price : i32,
    pub boardposition : i32,
    pub owned : bool
}
impl t for Utility{}

#[derive(Debug)]
pub struct Special{
    name : String,
    boardposition : i32
}
//impl t for Special{}


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
