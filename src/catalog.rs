
#[derive(PartialEq, Debug, Clone)]
pub enum Color {
    Red,
    Green,
    White,
    Black
}
#[derive(Debug, Clone)]
pub struct Footwear {
    pub brand: String,
    pub color: Color,
    pub shoe_size: usize
}
#[derive(Debug, Clone)]
pub struct TShirt {
    pub brand: String,
    pub color: Color,
    pub tshirt_size: usize
}