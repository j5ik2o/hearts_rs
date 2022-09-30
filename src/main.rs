// use rand::Rng;
use rand::seq::SliceRandom;

// Total number of games
const NUM_GAMES: usize = 1;

// Number of cards in each suit: 2-10, J, Q, K and A
const NUM_KC: usize = 13;

// Number of all cards
const NUM_CARDS: usize = NUM_KC * 4;

// Number of players: HEARTS expects to be played by four players.
const NUM_PLAYERS: usize = 4;


fn main() {

    // Making instances of 4 players and store the objects in Vec
    let mut players = Vec::new();
    for _ in 0..NUM_PLAYERS {
        let hands: [i32; NUM_KC] = [-1; NUM_KC];
        players.push(RandomAgent{hands: hands});
    }

    // Playing NUM_GAMES times game
    for _ in 1..=NUM_GAMES {

        // Deal cards to each player
        deal_cards(&mut players);
        
        for i in 0..NUM_PLAYERS {
            println!("{:?}", players[i].hands);
        }
        
    }
    
}


fn deal_cards(players: &mut Vec<RandomAgent>) {

    let mut v: Vec<i32> = (0..NUM_CARDS as i32).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    
    for i in 0..NUM_PLAYERS {
        players[i].set_hands(&v[(i * NUM_KC)..((i+1) * NUM_KC)]);
    }
    
}


struct RandomAgent {
    hands: [i32; NUM_KC],
}


impl RandomAgent {

    fn set_hands(&mut self, cards: &[i32]) {
        self.hands = cards.try_into().unwrap();
    }
    
}
