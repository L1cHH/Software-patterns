#[derive(Clone)]
pub struct ForestBuilderPage {
    pub is_weather_cb_checked : bool,
    pub input_area: String

}

impl Default for ForestBuilderPage {
    fn default() -> Self {
        Self {
            is_weather_cb_checked: false,
            input_area : String::new()
        }
    }
}