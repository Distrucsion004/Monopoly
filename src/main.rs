use rand::Rng;
fn main() {
    let board: [String; 40] = ["Go".to_string(), "Mediteranian Avenue".to_string(), "Community Chest".to_string(),
                                "Baltic Avenue".to_string(), "Income Tax".to_string(), "Reading Railroad".to_string(),
                                "Oriental Avenue".to_string(), "Chance".to_string(), "Vermont Avenue".to_string(),
                                "Connecticut Avenue".to_string(), "Jail".to_string(), "St. Charles Place".to_string(),
                                "Electric Company".to_string(), "States Avenue".to_string(), "Virginia Avenue".to_string(),
                                "Pensylavania Avenue".to_string(), "St. James Place".to_string(), "Community Chest".to_string(),
                                "Tenessee Avenue".to_string(), "New York Avenue".to_string(), "Free Parking".to_string(),
                                "Kentucky Avenue".to_string(), "Chance".to_string(), "Indian Avenue".to_string(),
                                "Illinois Avenue".to_string(), "B & O Railroad".to_string(), "Atlantic Avenue".to_string(),
                                "Ventnor Avenue".to_string(), "Water Works".to_string(), "Marvin Gardens".to_string(),
                                "Go to Jail".to_string(), "Pacific Avenue".to_string(), "North Carolina Avenue".to_string(),
                                "Community chest".to_string(), "Pennsylvania Avenue".to_string(), "Short Line".to_string(),
                                "Chance".to_string(), "Park Place".to_string(), "Luxury Tax".to_string(), "BoardWalk".to_string()];
    let mut dice = Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
    };
    
    println!("{}", board[1]);
    
    
}

struct Dice{
    d1 : i8,
    d2 : i8,
}
impl Dice{
    fn roll(mut self) -> Dice{
        self = Dice{
        d1 : rand::thread_rng().gen_range(1..7),
        d2 : rand::thread_rng().gen_range(1..7)
        };
        return self
    }
}

