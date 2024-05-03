


#[derive(Clone)]
pub struct ForestBuilderPage {
    pub is_rainy_weather_added : bool,
    pub is_sunny_weather_added : bool,
    pub is_gloomy_weather_added : bool,
    pub is_without_weather : bool,
    pub input_area: String

}

impl Default for ForestBuilderPage {
    fn default() -> Self {
        Self {
            is_rainy_weather_added: false,
            is_sunny_weather_added: false,
            is_gloomy_weather_added: false,
            is_without_weather: false,
            input_area : String::new()
        }
    }
}