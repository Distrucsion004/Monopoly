use crate::data::{Space, SpaceType, PropTypes};
use std::{collections::HashMap};

//generates a default player
pub fn initialize_player(na : i8) -> Player{
    let p = Player{
        number: na,
        money : 1500,
        boardposition :0,
        railroads : Vec::new(),
        utilities : Vec::new(),
        props : Vec::new(),
        mortgaged: HashMap::new(),
        in_jail : false,
        get_out_of_jail: 0
    };
    return p
}
#[derive(Debug)]
#[derive(Clone)]

pub struct Player{
    pub number : i8,
    pub money: i32,
    pub boardposition: i32,
    pub railroads: Vec<String>,
    pub utilities: Vec<String>,
    pub props: Vec<String>,
    pub mortgaged: HashMap<String, i8>,
    pub in_jail: bool,
    pub get_out_of_jail: i8
}

impl Player{
    
    pub fn take_money(&mut self, x : i32) -> (){
        self.money = self.money - x;
    }
    pub fn add_money(&mut self, x : i32) -> (){
        self.money = self.money + x;
    }

    //moves a number of spaces on the board, usually the ones shown by the dice and automatically activates go 
    pub fn dice_move(&mut self, x : i8) -> (){
        
        let y = self.boardposition + x as i32;
        if y <= 39{
            self.boardposition = y;
        }
        else {
            self.boardposition = y - 40;
            self.add_money(200);
            if self.boardposition != 0{
            println!("You passed by Go\n200 $ have been added to your wallet")
            }
        }
        
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
    pub fn buy_prop(&mut self,  r: &mut Space)->(){
        if r.price <= self.money{
        match r.kind{
        SpaceType::Prop =>self.add_prop(&r),
        SpaceType::Railroad => self.add_railroad(&r),
        SpaceType::Utility => self.add_utility(&r),
        _ => panic!("Error")
        }
        self.update_ownership(r);
        self.take_money(r.price);
        }
        else {println!{"Not enough money!"};}
    }
    pub fn go_to_jail(&mut self) ->(){
        self.in_jail = true 
    }
    pub fn leave_jail(&mut self) ->(){
        self.in_jail = false 
    }
    /*pub fn check_props(&self) ->(){
        println!("{:?}", self.props);
        println!("{:?}", self.railroads);
        println!("{:?}", self.utilities);}*/
    pub fn update_ownership(&self, mut h : &mut Space,) -> (){
        if h.owned == false{
            h.owned = true;
            h.owner = self.number}
        else {h.owned = false;}}


    pub fn check_set(&self, check : &Space, x:&[Space;40])-> bool{
        let col = &check.class;
        let mut valid = 0;
        for k in 0..self.props.len(){   
            
                for l in 0..40{
                    if x[l].name == self.props[k].to_string() && &x[l].class == col{
                        valid = valid+1;
                        
                    }
                }
            
        }
        if (col == &PropTypes::Cherry || col == &PropTypes::Blue) && valid == 2{
            return true
        }
        if valid == 3{
            return true
        }
        return false
    }

    pub fn payToLeave(&mut self) -> (){
        self.take_money(50);
        self.in_jail  = false;
    }
    
    
    
    }

    


