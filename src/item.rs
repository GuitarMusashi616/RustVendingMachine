#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Item {
    Coke,
    Sprite,
    Pepsi,
    Cheetos,
}

impl Item {
    pub fn get_cost(&self) -> u32 {
        match self {
            Item::Coke => 90,
            Item::Sprite => 85,
            Item::Pepsi => 75,
            Item::Cheetos => 135,
        }
    }
}