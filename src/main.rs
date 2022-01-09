#![allow(dead_code, unused_imports)]
mod coin;
mod item;
mod stockable;
mod vending_machine;

use coin::Coin::{Dime, Nickel, Penny, Quarter};
use item::Item::{Cheetos, Coke, Pepsi, Sprite};
use vending_machine::VendingMachine;

use crate::stockable::Unstocked;

fn main() {
    let vm = 
    VendingMachine::<Unstocked>::new()
    .store(Coke, 5)
    .store(Pepsi, 4)
    .store(Sprite, 3)
    .store(Cheetos, 6)
    .lock("1337");

    let mut vm = 
    vm
    .insert(Quarter)
    .insert(Quarter)
    .insert(Quarter)
    .insert(Quarter)
    .select(Coke)
    .buy();

    assert_eq!(vm.grab_item().unwrap(), Coke);
    assert_eq!(vm.grab_change(), vec![Dime]);
}
