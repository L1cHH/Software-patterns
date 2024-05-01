use std::collections::HashMap;
use crate::catalog::{TShirt, Footwear, Color};
#[derive(Debug, Clone)]
struct Location {
    city: String,
    street: String
}
#[derive(Debug, Clone)]
struct ClothingStore {
    footwear_in_stock: HashMap<String, Footwear>,
    t_shirts_in_stock: HashMap<String, TShirt>,
    location: Option<Location>,
    shop_area: usize
}
impl ClothingStore {
    fn builder() -> StoreBuilder {
        StoreBuilder {
            footwear_in_stock: HashMap::new(),
            t_shirts_in_stock: HashMap::new(),
            location: None,
            shop_area: None
        }
    }

    fn get_particular_footwear(&mut self, name: String, brand: String, color: Color, shoe_size: usize) -> &Footwear {
        match !self.footwear_in_stock.contains_key(&name) {
            true => {
                let shoe = self.footwear_in_stock.insert(name, Footwear {brand, color, shoe_size}).unwrap();
                return &shoe
            },
            false => {
                let shoe = self.footwear_in_stock.get(&name).unwrap();
                return &shoe
            }
        };



    }

    fn get_particular_tshirt(&mut self, brand: String, color: Color, tshirt_size: usize) -> &TShirt {
        if let Some(tshirt) = self.t_shirts_in_stock.iter_mut().find(|&tshirt| tshirt.brand == brand && tshirt.color == color && tshirt.tshirt_size == tshirt_size) {
            return tshirt
        };

        let new_tshirt = TShirt {brand, color, tshirt_size};
        self.t_shirts_in_stock.push(new_tshirt);
        self.t_shirts_in_stock.last_mut().unwrap()
    }
}

struct StoreBuilder {
    footwear_in_stock: HashMap<String, Footwear>,
    t_shirts_in_stock: HashMap<String, TShirt>,
    location: Option<Location>,
    shop_area: Option<usize>
}
impl StoreBuilder {
    fn add_location(&mut self, location: Location) -> &mut Self {
        self.location = Some(location);
        self
    }

    fn add_shop_area(&mut self, shop_area: usize) -> &mut Self {
        self.shop_area = Some(shop_area);
        self
    }

    fn build(&mut self) -> ClothingStore {
        ClothingStore {
            footwear_in_stock: self.footwear_in_stock.clone(),
            t_shirts_in_stock: self.t_shirts_in_stock.clone(),
            location: self.location.clone(),
            shop_area: self.shop_area.unwrap_or(0)
        }
    }
}