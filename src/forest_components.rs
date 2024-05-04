use std::fmt::format;
use std::rc::Rc;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Color {
    Green,
    Brown,
    Unknown
}
#[derive(Debug, Clone)]
pub struct Location {
    x: i32,
    y: i32
}
impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TreeKind {
    name: String,
    color: Color
}
impl TreeKind {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color
        }
    }
}
#[derive(Debug, Clone)]
pub struct Tree {
    location: Location,
    tree_kind: Rc<TreeKind>
}
impl Tree {
    pub fn new(location: Location, tree_kind: Rc<TreeKind>) -> Self {
        Self {
            location,
            tree_kind
        }
    }

    pub fn grown(&self) {
        let location = &self.location;
        let tree_kind = &self.tree_kind;
        let name = &tree_kind.name;
        let color = &tree_kind.color;
        println!("Было выращено дерево => \r
            Вид: {},\r
            Цвет: {:?},\r
            Местоположение: {:?}",
            name, color, location)
    }
}
#[derive(Default, Debug, Clone)]
pub enum Weather {
    Rainy,
    #[default]
    Sunny,
    Gloomy
}