use std::collections::HashMap;
use crate::stockable::{Stockable, Stocked, Unstocked};
use crate::{
    coin::Coin,
    item::Item,
};

pub struct VendingMachine<T: Stockable> {
    state: T,
    selected: Option<Item>,
    output_slot: Option<Item>,
    change_receptacle: Vec<Coin>,
    inventory: HashMap<Item, u32>,
}

impl VendingMachine<Unstocked> {
    pub fn new() -> Self {
        return Self {
            state: Unstocked {},
            selected: None,
            output_slot: None,
            change_receptacle: Vec::new(),
            inventory: HashMap::new()
        };
    }

    fn add(&mut self, snack: Item) {
        let entry = self.inventory.entry(snack).or_insert(1);
        *entry += 1;
    }

    pub fn store(mut self, snack: Item, count: u32) -> Self {
        println!("Storing {}x {:?}", count, snack);
        self.add(snack);
        self
    }

    pub fn lock(self, password: &str) -> VendingMachine<Stocked> {
        println!("Locked with password: {}", password);
        VendingMachine {
            state: Stocked {password: password.to_string()},
            selected: self.selected,
            output_slot: self.output_slot,
            change_receptacle: self.change_receptacle,
            inventory: self.inventory,
        }
    }
}

impl VendingMachine<Stocked> {
    pub fn insert(mut self, coin: Coin) -> Self {
        self.change_receptacle.push(coin);
        println!("Vending Machine now contains {} cents", self.count_balance());
        self
    }

    fn count_balance(&self) -> u32 {
        self.change_receptacle
            .iter()
            .map(|x|x.get_value())
            .sum()
    }

    pub fn select(mut self, item: Item) -> Self {
        self.selected = Some(item);
        println!("{:?} selected", item);
        self
    }

    fn has_enough_money(&self, selected: Item) -> bool {
        self.count_balance() >= selected.get_cost()
    }

    fn item_in_inventory(&self, selected: Item) -> bool {
        self.inventory.contains_key(&selected)
    }

    fn remove_item(&mut self, selected: Item) {
        if self.inventory.contains_key(&selected) { 
            self.inventory.insert(selected, self.inventory[&selected] - 1);
        }
    }

    fn remove_coins(&mut self, selected: Item) {
        let change_amount = self.count_balance() - selected.get_cost();
        println!("{} cents returned as change", change_amount);
        let change = Coin::get_coins(change_amount);
        self.change_receptacle = change;
    } 

    pub fn buy(mut self) -> Self {
        // if has enough money and item in inventory
        // remove item, remove money, dispense item
        let selected = match self.selected {
            Some(i) => i,
            _ => return self,
        };

        if !self.has_enough_money(selected) || !self.item_in_inventory(selected) {
            println!("Could not buy item, not enough money or out of stock");
            return self;
        }
        println!("Successfully bought {:?}", selected);

        self.remove_item(selected);
        self.remove_coins(selected);

        self.output_slot = self.selected.take();
        self
    }

    pub fn unlock(self, password: &str) -> VendingMachine<Unstocked> {
        assert_eq!(self.state.password, password); 
        println!("Vending Machine unlocked, can now be restocked");
        VendingMachine {
            state: Unstocked {},
            selected: self.selected,
            output_slot: self.output_slot,
            change_receptacle: self.change_receptacle,
            inventory: self.inventory,
        }
    }

    pub fn grab_item(&mut self) -> Option<Item> {
        self.output_slot.take()
    }

    pub fn grab_change(&mut self) -> Vec<Coin> {
        self.change_receptacle.drain(..).collect()
    }
}
