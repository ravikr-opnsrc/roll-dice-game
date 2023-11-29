use rand::Rng;

struct Player {
    name: u32,
    score: u32,
}

impl Player {
    // make a new player
    fn new(name: u32, score: u32) -> Self {
        Self { name, score }
    }

    // roll dice
    fn roll_dice() -> u32 {
        rand::thread_rng().gen_range(1..=6)
    }

    // update score of the player
    fn update_score(&mut self) {
        let mut roll = Self::roll_dice();
        self.score += roll;
        // let mut six_counts = 0;
        while roll == 6 {
            // six_counts +=1;
            // print!("P{} got 6",self.name );

            roll = Self::roll_dice();
            // println!("   next roll for P{}, is {}",self.name,roll);
            self.score += roll;
            // if six_counts == 3 {
            //     self.score -=18;
            // }
        }
    }

    // read score of a player
    fn score(&self) -> u32 {
        self.score
    }
}

fn main() {
    const TARGET_SCORE: u32 = 1000;

    let mut player_01 = Player::new(1, 0);
    let mut player_02 = Player::new(2, 0);
    let mut player_03 = Player::new(3, 0);
    // let mut player_04 = Player::new(3, 0);

    let mut matches = 0; // count no of rolls
    loop {
        println!("P1 {}", player_01.score());
        println!("P2 {}", player_02.score());
        println!("P3 {}", player_03.score());
        // println!("P4 {}", player_04.score());

        player_01.update_score();
        player_03.update_score();
        player_02.update_score();
        // player_04.update_score();

        println!();
        println!();

        if player_01.score() >= TARGET_SCORE {
            println!("Player {} wins", player_01.name);
            break;
        }
        if player_02.score() >= TARGET_SCORE {
            println!("Player {} wins", player_02.name);
            break;
        }
        if player_03.score() >= TARGET_SCORE {
            println!("Player {} wins", player_03.name);
            break;
        }
        /* STRANGE! P4 never wins !!  */
        // if player_04.score() >= TARGET_SCORE {
        //     println!("Player {} wins", player_04.name);
        //     break;
        // }

        matches += 1;

        /* extreamly difficult to get same score!  */
        // if player_01.score() == player_02.score() && player_02.score()== player_03.score() {
        //     println!("P1: {}, P2: {}, P3: {}", player_01.score(), player_02.score(), player_03.score());break;}
    }
    println!("rolls {}", matches);
}
