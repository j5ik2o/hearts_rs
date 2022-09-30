use rand::Rng;
use rand::seq::SliceRandom;

// Total number of games
const NUM_GAMES: usize = 1;

// Number of cards in each suit: 2-10, J, Q, K and A
const NUM_KC: usize = 13;

// Number of all cards
const NUM_CARDS: usize = NUM_KC * 4;

// Number of players; HEARTS expects to be played by four players.
const NUM_PLAYERS: usize = 4;


fn main() {

    // Making instances of 4 agents and store the objects in Vec.
    let mut players = Vec::new();
    for _ in 0..NUM_PLAYERS {
        let hands: [i32; NUM_KC] = [-1; NUM_KC];
        players.push(RandomAgent{hands: hands});
    }

    // Having agents play the card game "Hearts" NUM_GAMES times.
    for _ in 1..=NUM_GAMES {

        // Dealing cards to each agent.
        let dealt_cards = deal_cards(&mut players);

        // Getting the playing sequence in the first trick based on agents' hands.
        // (the agent who has C-2 is the leading player in the initial trick).
        let mut idx = 0;
        for (i, val) in dealt_cards.iter().enumerate() {
            if *val == 0 {
                idx = i;
                break;
            }
        }
        let mut winner = (idx as i32) / (NUM_KC as i32);
        
        for i in 0..NUM_PLAYERS {
            println!("{:?}", players[i].hands);
        }
        
        for trick in 0..NUM_KC {

            let agent_order = determine_agent_order(winner);
            println!("{:?}", agent_order);
            
            for turn in 0..NUM_PLAYERS {

                let card = players[turn].select_card();
                print!("{}, ", card);
                
            }
            println!("");
            
        }
        
        for i in 0..NUM_PLAYERS {
            println!("{:?}", players[i].hands);
        }
        
    }
    
}


fn deal_cards(players: &mut Vec<RandomAgent>) -> Vec<i32> {

    let mut v: Vec<i32> = (0..NUM_CARDS as i32).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    
    for i in 0..NUM_PLAYERS {
        players[i].set_hands(&v[(i * NUM_KC)..((i+1) * NUM_KC)]);
    }

    return v;
    
}


fn determine_agent_order(winner: i32) -> [i32; NUM_PLAYERS] {

    let mut order: [i32; NUM_PLAYERS] = [-1; NUM_PLAYERS];

    for i in 0..NUM_PLAYERS {
        if winner + (i as i32) < (NUM_PLAYERS as i32) {
            order[i] = winner + (i as i32);
        } else {
            order[i] = winner + (i as i32) - (NUM_PLAYERS as i32);
        }
    }

    return order;
    
}


struct RandomAgent {
    hands: [i32; NUM_KC],
}


impl RandomAgent {

    fn set_hands(&mut self, cards: &[i32]) {
        self.hands = cards.try_into().unwrap();
    }

    fn select_card(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        loop {
            let card_index = rng.gen_range(0..NUM_KC);
            if self.hands[card_index] != -1 {
                let card = self.hands[card_index];
                self.hands[card_index] = -1;
                return card;
            }
        }
    }
    
}
