use crate::data::{Space};

pub fn initialize_player() -> Player{
    let mut p = Player{
        money : 1500,
        boardposition :0,
        railroads : Vec::new(),
        utilities : Vec::new(),
        props : Vec::new()
    };
    return p
}
#[derive(Debug)]
pub struct Player{
    pub money: i32,
    pub boardposition: i32,
    pub railroads: Vec<String>,
    pub utilities: Vec<String>,
    pub props: Vec<String>
}

impl Player{
    pub fn take_money(&mut self, x : i32) -> (){
        self.money = self.money - x;
    }
    pub fn add_money(&mut self, x : i32) -> (){
        self.money = self.money + x;
    }

    pub fn dice_move(&mut self, x : i8) -> (){
        self.boardposition = self.boardposition + x as i32;
    }

    pub fn add_railroad(&mut self, r : &Space) -> (){
        self.railroads.push(r.name.to_string());
        
    }
    pub fn add_utility(&mut self, r : &Space) -> (){
        self.utilities.push(r.name.to_string());
    }
    pub fn add_prop(&mut self, r : &Space) -> (){
        self.props.push(r.name.to_string());
    }
    

    
}
pub fn update_ownership(mut h : &mut Space,) -> (){
    if h.owned == false{
        h.owned = true;}
    else {h.owned = false;}
}
