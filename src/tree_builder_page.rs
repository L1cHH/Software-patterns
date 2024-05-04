#[derive(Clone)]
pub struct TreeBuilderPage {
    pub tree_kind_name : String,
    pub color_value : String,
    pub location_x : String,
    pub location_y : String
}
impl Default for TreeBuilderPage {
    fn default() -> Self {
        Self {
            tree_kind_name: String::new(),
            color_value: String::new(),
            location_x: String::new(),
            location_y: String::new(),
        }
    }
}