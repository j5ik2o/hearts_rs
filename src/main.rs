// Total number of games
const NUM_GAMES:i32 = 1000;

// Number of suits: Spade, Heart, Diamond and Club
const NUM_SUITS:i32 = 4;

const CLUB:i32 = 0;
const DIA:i32 = 1;
const SPADE:i32 = 2;
const HEART:i32 = 3;

// Number of cards in each suit: 2-10, J, Q, K and A
const NUM_KC:i32 = 13;

// Number of all cards
const NUM_CARDS:i32 = NUM_SUITS * NUM_KC;

// Number of players: Hearts is played by four players.
const NUM_PLAYERS:i32 = 4;

const C_2:i32 = 0;
const S_Q:i32 = SPADE * NUM_KC + (NUM_KC - 3);
const S_K:i32 = SPADE * NUM_KC + (NUM_KC - 2);
const S_A:i32 = SPADE * NUM_KC + (NUM_KC - 1);
const H_2:i32 = HEART * NUM_KC;

fn main() {
    
    for i in 1..=NUM_GAMES {
        
        println!("{}", i);
        
    }
    
}
