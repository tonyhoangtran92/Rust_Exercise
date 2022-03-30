fn main () {
    decimals(Coin::Solana);
}

enum Coin {
    Solana, 
    Ethereum, 
    Near
}

fn decimals(coin: Coin) -> u8 {
    match coin {
        Coin:: Solana => { println!("Solana match"); 1},
        Coin:: Ethereum => 100,
        Coin::Near => 200
    }
}