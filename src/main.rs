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

    // 4人のプレイヤー（RandomAgent）をインスタンス化し、オブジェクトをVecに格納する
    let mut players = Vec::new();
    for _ in 0..NUM_PLAYERS {
        let hands: [i32; NUM_KC] = [-1; NUM_KC];
        players.push(RandomAgent{hands: hands});
    }

    // NUM_GAMES だけゲームをプレイする
    for _ in 1..=NUM_GAMES {
        play_one_game(players);
    }
    
}


fn play_one_game(players: Vec<RandomAgent>) {

    // 各プレイヤーにカードを配る
    distribute_cards(players);
    
}


fn distribute_cards(players: Vec<RandomAgent>) {

    let mut v: Vec<i32> = (0..NUM_CARDS as i32).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);

    for i in 0..NUM_PLAYERS {
        // println!("{:?}", &v[0..13]);
        players[i as usize].set_hands(&v[0..13]);
        
        // println!("{:?}", v.split_at(NUM_KC).0);
        // let cards = vec!(v.split_at(NUM_KC as usize).0);
        // println!("{:?}", cards)
        // players[i as usize].set_hands(v.split_at(NUM_KC).0);
        // players[i as usize].set_hands(cards);
    }
    
}


struct RandomAgent {
    hands: [i32; NUM_KC],
}


impl RandomAgent {

    fn set_hands(&mut self, cards: &[i32]) {
        for i in 0..NUM_KC {
            self.hands[i] = cards[i];
        }
    }
    
}
