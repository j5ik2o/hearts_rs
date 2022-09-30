use rand::Rng;
use rand::seq::SliceRandom;

// Total number of games
const NUM_GAMES: i32 = 1;

// Number of cards in each suit: 2-10, J, Q, K and A
const NUM_KC: i32 = 13;

// Number of all cards
const NUM_CARDS: i32 = NUM_KC * 4;

// Number of players: HEARTS expects to be played by four players.
const NUM_PLAYERS: i32 = 4;


fn main() {

    let players = Vec::new();
    for _ in 0..NUM_PLAYERS {
        let hands: Vec<i32> = (0..NUM_KC).collect();
        players.push(RandomAgent{hands: hands});
    }
    
    for i in 1..=NUM_GAMES {
        play_one_game(i, &players);
    }
    
}


fn play_one_game(game_number: i32, players: &Vec<RandomAgent>) {
    
    distribute_cards(players)
    
}


fn distribute_cards(players: &Vec<RandomAgent>) {
    
    let mut v: Vec<i32> = (0..NUM_CARDS).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);

    for i in 0..NUM_PLAYERS {
        players[i as usize].set_hands(v.split_at(NUM_KC as usize))
    }
    
}


struct RandomAgent {
    hands: Vec<i32>,
}


impl RandomAgent {

    fn set_hands(&self, cards: &Vec<i32>) {
        self.hands = cards;
    }
    
}
