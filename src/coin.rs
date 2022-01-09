#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn get_value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    pub fn get_coins(mut balance: u32) -> Vec<Coin> {
        let mut result: Vec<Coin> = Vec::new();
        while balance > 0 {
            let coin = Coin::get_largest_coin_for_rem_change(balance);
            result.push(coin);
            balance -= coin.get_value();
        }
        result
    }

    fn get_largest_coin_for_rem_change(rem_change: u32) -> Coin {
        match rem_change {
            25.. => Coin::Quarter, 
            10.. => Coin::Dime,
            5.. => Coin::Nickel,
            _ => Coin::Penny,
        }
    }
}
