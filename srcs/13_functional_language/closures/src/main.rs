#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(| | self.most_stocked())
    }
    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1
            }
        }
        
        if num_red >= num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}


fn main() {
    let store = Inventory { shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red] };
    
    let user1_preference = Some(ShirtColor::Red);
    let give_away1 = store.giveaway(user1_preference);
    println!("The user with preference {:?} gets {:?}.", user1_preference, give_away1);
    
    let user2_preference = None;
    let give_away2 = store.giveaway(user2_preference);
    print!("The user with preference {:?} gets {:?}\n", user2_preference, give_away2);
}
