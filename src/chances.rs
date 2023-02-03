use core::prelude;

use crate::{player::{Player, self}, data::{Space, find_rent, PropTypes, SpaceType, rent_match, props, jail_init, Jail}};

pub fn chances() -> [&'static str; 11]{
    let chances : [&'static str; 11] = ["Advance to Boardwalk", "Advance to Go", "Advance to St. Charles Place", "Advance to the nearest railroad",
                                "Advance to nearest utility, if owned roll the dice and pay 10X what you roll" , "Bank pays you $10",
                                "Get out of jail free card", "Go back 3 spaces", "Go to Jail", "For each house owned pay $25 and for each hotel $100",
                                "Speeding fine $15"/*, "Go to Reading Railroad", "You have been elected Chairman of the Board. Pay each player $50",
                                "Your building loan matures. Collect $150", "Advance to the nearest railroad",
                                "Advance to Illinois Avenue"*/];
    return chances;
    }

pub fn ch0(pl : &mut Player) -> (){
    if pl.boardposition == 39{   
    }
}

pub fn ch1(pl : &mut Player) -> (){
    pl.dice_move(40 - pl.boardposition as i8);
    println!("$200 have been added to your wallet!")
}

pub fn ch2(mut pl : &mut Player) -> (){
    pl.boardposition = 0;
}

pub fn ch3(lu:&mut Vec<Player>,pl : usize, li:&mut [Space; 40]) ->(){
    let o =lu[pl].boardposition;
    let mut p =0;
    if o % 5 == 0 && o%2 !=0{
    }
    else{
        if o == 0{
            lu[pl].dice_move(5);
            p = 5;
        }
        else if o%2 == 0 &&  o % 5 == 0{
            lu[pl].dice_move(((o + 5) -o) as i8);
            p = (o / 5+ 1 * 5 - o) as i32;
        }
        
        else if o>5{
            if (o / 5 + 1) * (5 ) % 2 !=0{
            lu[pl].dice_move(((o / 5 + 1) * (5 )-o) as i8);
            p = ((o / 5 + 1) * (5 )) as i32;}
            else{
                lu[pl].dice_move(((o / 5 + 2) * (5 )-o) as i8);
            p = ((o / 5 + 2) * (5 )) as i32;}
            
        }
        else {
            lu[pl].dice_move((5-o) as i8);
            p = 5;
        }
    }
    if li[p as usize].owned == true{
        
        let you = p as i32;
        let ren = rent_match(lu, you, li);
        //println!("{}", ren);
        let idk =find_rent( you , li, ren);
        //println!("{}", idk);
        lu[pl].take_money(idk);
        //println!{"{:?}",lu[pl] };
        lu[(li[p as usize].owner -1) as usize].add_money(idk);
        //println!{"{:?}",lu[(li[p as usize].owner -1) as usize] };
    }
}

pub fn ch4 (lu:&mut Vec<Player>,pl: usize, li :&mut [Space; 40]) ->(){
    let posi = lu[pl].boardposition;
    if posi <= 12 || posi > 28 {
        if posi <= 12{
            lu[pl].dice_move( (12 - posi ) as i8 );
        }
        else{ 
            lu[pl].dice_move((12 +  40 - posi) as i8);
        }
    }
    else {lu[pl].dice_move((28 - posi) as i8)};
    
    if li[posi as usize].owned == true{
        //println!{"Reached Check"};
        let you = posi as i32;
        let ren = rent_match(lu, you, li);
        let idk =find_rent( you , li, ren);
        lu[pl].take_money(idk);
        lu[(li[posi as usize].owner -1) as usize].add_money(idk);
    }
}
 
pub fn ch5(pl: &mut Player){
    pl.add_money(10);
    println!("New balance: {}", pl.money);
}

pub fn ch6(pl : &mut Player){
    pl.get_out_of_jail += 1;
    println!("Number of get out of jail free cards: {}", pl.get_out_of_jail);
}

pub fn ch7(pl: &mut Player){
    pl.dice_move(-3);
    println!("Moved back to {}", props().0[pl.boardposition as usize]);
}

pub fn ch8 (pl: &mut Player, j :&mut Vec<Jail>)->(){
    j.push(jail_init(&pl)); 
    pl.boardposition = 10;
    pl.go_to_jail();
}

pub fn ch9 (pl : &mut Player, dat : &mut [Space;40]) -> (){
    let mut housing = 0;
    for ui in 0..pl.props.len(){
        for w in 0..40{
            if dat[w].name == pl.props[ui]{
                    housing += dat[w].houses * 25;
                    
                if dat[w].hotel == true{
                    housing += 100;
                    break
                }
                break
            }
        }
    }
    println!("${} have been deducted from your wallet", housing);
    pl.take_money(housing as i32);
}

pub fn ch10 (pl: &mut Player) -> (){
    pl.take_money(15);
    println!("New balance: {}", pl.money);
}