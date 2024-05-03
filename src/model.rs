use std::collections::HashSet;
use std::rc::Rc;
use crate::forest_components::{Color, TreeKind, Tree, Location, Weather};

pub struct Forest {
    tree_kinds: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
    forest_area: usize,
    weather: Option<Weather>,
}
impl Forest {

    pub fn builder() -> ForestBuilder {
        ForestBuilder {
            tree_kinds: HashSet::new(),
            trees: Vec::new(),
            forest_area: None,
            weather: None
        }
    }
    pub fn plant_tree(&mut self, location: Location, name: String, description: String, color: Color) {
        let tree_kind = TreeKind::new(name, description, color);

        self.tree_kinds.insert(Rc::new(tree_kind.clone()));

        let tree = Tree::new(location, self.tree_kinds.get(&tree_kind).unwrap().clone());
        self.trees.push(tree);
    }
    
    pub fn tress_grown(&self) {
        for tree in &self.trees {
            tree.grown()
        }
    }
}

struct ForestBuilder {
    tree_kinds: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
    forest_area: Option<usize>,
    weather: Option<Weather>,
}

impl ForestBuilder {
    fn add_weather(&mut self, weather: Weather) -> &mut Self {
        self.weather = Some(weather);
        self
    }

    fn add_area(&mut self, area: usize) -> &mut Self {
        self.forest_area = Some(area);
        self
    }
    fn build(&mut self) -> Forest {
        Forest {
            tree_kinds: self.tree_kinds.clone(),
            trees: self.trees.clone(),
            forest_area: self.forest_area.unwrap_or(100),
            weather: self.weather.clone()
        }
    }
}
