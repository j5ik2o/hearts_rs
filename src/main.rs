use rand::Rng;
use rand::seq::SliceRandom;

// Total number of games
const NUM_GAMES: i32 = 1;

// Number of cards in each suit: 2-10, J, Q, K and A
const NUM_KC: i32 = 13;

// Number of all cards
const NUM_CARDS: i32 = 4 * NUM_KC;

// Number of players: HEARTS expects to be played by four players.
const NUM_PLAYERS: i32 = 4;


fn main() {
    
    for i in 1..=NUM_GAMES {
        play_one_game(i);
    }
    
}


fn play_one_game(game_number: i32) {
    
    distribute_cards()
    
}


fn distribute_cards() {
    
    let mut v: Vec<i32> = (0..NUM_CARDS).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    println!("{:?}", v.split_at(NUM_KC as usize));
    
}
