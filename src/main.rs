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

const C_2: i32 = 0;

const CLUB: i32 = 0;
const DIA: i32 = 1;
const SPADE: i32 = 2;
const HEART: i32 = 3;


fn main() {

    // Making instances of 4 agents and store the objects in Vec.
    let mut players = Vec::new();
    for _ in 0..NUM_PLAYERS {
        let hand: [i32; NUM_KC] = [-1; NUM_KC];
        players.push(RandomAgent{hand: hand});
    }

    // Letting agents play the card game "Hearts" NUM_GAMES times.
    for _ in 1..=NUM_GAMES {

        // Dealing cards to each agent.
        let dealt_cards = deal_cards(&mut players);

        // Getting the playing sequence in the first trick based on agents' hands.
        // (the agent who has C-2 is the leading player in the initial trick).
        let mut idx = 0;
        for (i, val) in dealt_cards.iter().enumerate() {
            if *val == C_2 {
                idx = i;
                break;
            }
        }
        let mut winner = (idx as i32) / (NUM_KC as i32);

        // initializing the flag of "breaking heart"".
        let mut bh_flag = false;

        for trick in 0..NUM_KC {

            let agent_order = determine_agent_order(winner);
            
            let mut card_sequence: [i32; NUM_PLAYERS] = [-1; NUM_PLAYERS];
            
            for turn in 0..NUM_PLAYERS {
                
                let playing_agent = agent_order[turn] as usize;

                // Letting the agent choose a card.
                let mut card;
                loop {
                    card = players[playing_agent].select_card();
                    if is_valid_card(&players[playing_agent].hand, &card_sequence, card, trick, bh_flag) {
                        break;
                    }
                }
                players[playing_agent].update_hand(card);

                card_sequence[turn] = card;
                
                // When a heart is played for the first time in a game, setting the flag to true.
                if !bh_flag && get_suit(card) == HEART {
                    bh_flag = true;
                }
                
            }

            // The winner of the current trick becomes the leading player of the next trick.
            winner = determine_winner(&agent_order, &card_sequence);
            
        }
        
    }
    
}


fn deal_cards(players: &mut Vec<RandomAgent>) -> Vec<i32> {

    let mut v: Vec<i32> = (0..NUM_CARDS as i32).collect();
    loop {
        let mut rng = rand::thread_rng();
        v.shuffle(&mut rng);

        // Prohibiting hearts from appearing 13 times in a row.
        let mut count = 0;
        for i in 0..NUM_CARDS {
            if get_suit(v[i]) == HEART {
                count += 1;
            } else {
                count = 0;
            }
        }
        if count < NUM_KC {
            break;
        }
    }
    
    for i in 0..NUM_PLAYERS {
        let cards = &v[(i * NUM_KC)..((i+1) * NUM_KC)];
        players[i].set_hand(&cards);
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


fn get_suit(card: i32) -> i32 {
    return card / (NUM_KC as i32);
}


fn is_valid_card(hand: &[i32; NUM_KC], card_sequence: &[i32; NUM_PLAYERS], card: i32, trick: usize, bh_flag: bool) -> bool {
    
    let leading_card = card_sequence[0];
    
    if leading_card == -1 {
        
        // In the first trick, only Club-2 can be the leading card.
        if trick == 0 && card != C_2 {
            return false
        }

        // In the first trick, each agent cannot play a heart.
        if trick == 0 && get_suit(card) == HEART {
            return false;
        }
        
        // If the leading player has only hearts, it is an exceptional case and the agent may lead with a heart.
        if get_suit(card) == HEART && !is_suit_in_hand(hand, CLUB) && !is_suit_in_hand(hand, DIA) && !is_suit_in_hand(hand, SPADE) {
            return true;
        }

        // Until breaking heart occurs, the leading player may not play a heart.
        if !bh_flag && get_suit(card) == HEART {
            return false;
        }
        
        return true;
        
    } else {

        // If an agent does not have a card of the same suit as the leading card, the agent play any card.
        if !is_suit_in_hand(hand, get_suit(leading_card)) {
            return true;
        }

        // Each agent must play a card of the same suit as the leading card.
        if get_suit(leading_card) == get_suit(card) {
            return true;
        }

        return false;
        
    }
    
}


fn determine_winner(agent_order: &[i32; NUM_PLAYERS], card_sequence: &[i32; NUM_PLAYERS]) -> i32 {

    let mut leading_card = card_sequence[0];
    let lc_suit = get_suit(leading_card);
    let mut winner = agent_order[0];

    // After a trick, the agent who has played the strongest card of the same suit as the leading card
    // is the winner of that trick.
    
    for (card, agent) in card_sequence.iter().zip(agent_order.iter()) {
        if lc_suit == get_suit(*card) && leading_card < *card {
            leading_card = *card;
            winner = *agent;
        }
    }
    return winner;
    
}


fn is_suit_in_hand(hand: &[i32; NUM_KC], suit: i32) -> bool {
    
    for h in hand {
        if *h != -1 && suit == get_suit(*h) {
            return true;
        }
    }
    return false;
    
}


struct RandomAgent {
    hand: [i32; NUM_KC],
}


impl RandomAgent {

    fn set_hand(&mut self, cards: &[i32]) {
        self.hand = cards.try_into().unwrap();
    }

    // Randomly selecting a card from the hand.
    fn select_card(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        loop {
            let card_index = rng.gen_range(0..NUM_KC);
            if self.hand[card_index] != -1 {
                return self.hand[card_index];
            }
        }
    }

    fn update_hand(&mut self, card: i32) {
        for i in 0..NUM_KC {
            if self.hand[i] == card {
                self.hand[i] = -1;
                break;
            }
        }
    }
    
}
