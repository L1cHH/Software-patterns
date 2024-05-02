use std::collections::HashMap;
use crate::catalog::{TShirt, Footwear, Color};
#[derive(Debug, Clone)]
struct Location {
    city: String,
    street: String
}
#[derive(Debug, Clone)]
struct ClothingStore {
    footwear_in_stock: Vec<Footwear>,
    t_shirts_in_stock: Vec<TShirt>,
    location: Option<Location>,
    shop_area: usize
}
impl ClothingStore {
    fn builder() -> StoreBuilder {
        StoreBuilder {
            footwear_in_stock: Vec::new(),
            t_shirts_in_stock: Vec::new(),
            location: None,
            shop_area: None
        }
    }

    fn get_particular_footwear(&mut self, brand: String, color: Color, shoe_size: usize) -> &Footwear {
        if let Some(shoe) = self.footwear_in_stock.iter().find(|shoe| shoe.brand == brand && shoe.color == color && shoe.shoe_size == shoe_size) {
            return shoe
        };

        let new_shoe = Footwear {brand, color, shoe_size};

        self.footwear_in_stock.push(new_shoe);

        self.footwear_in_stock.last().unwrap()
    }

    fn get_particular_tshirt(&mut self, brand: String, color: Color, tshirt_size: usize) -> &TShirt {
        if let Some(tshirt) = self.t_shirts_in_stock.iter().find(|tshirt| tshirt.brand == brand && tshirt.color == color && tshirt.tshirt_size == tshirt_size) {
            return tshirt
        };

        let new_tshirt = TShirt {brand, color, tshirt_size};

        self.t_shirts_in_stock.push(new_tshirt);

        self.t_shirts_in_stock.last().unwrap()
    }
}

struct StoreBuilder {
    footwear_in_stock: Vec<Footwear>,
    t_shirts_in_stock: Vec<TShirt>,
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