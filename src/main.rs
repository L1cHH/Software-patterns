use crate::forest_components::{Color, Location};
use crate::model::{Forest};
mod model;
mod forest_components;

fn main() {
    let mut forest = Forest::new();

    forest.plant_tree(Location::new(15, 20), "Дуб".to_string(), "Я Дуб!!!".to_string(), Color::Brown);

    forest.tress_grown()
}
