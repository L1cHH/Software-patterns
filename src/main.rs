use std::fmt::format;
use iced::{alignment, Alignment, Element, Length, Sandbox, Settings, Size, window};
use iced::Alignment::Center;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{button, Checkbox, column, row, Container, container, text, text_input, Column, Space};
use iced::widget::shader::wgpu::naga::FastIndexMap;
use crate::forest_components::{Color, Location, Weather};
use crate::model::{Forest};
use crate::forest_builder::ForestBuilderPage;
use crate::ForestState::ForestCreated;

mod model;
mod forest_components;
mod forest_builder;


fn main() -> iced::Result {
    ForestState::run(Settings {
        window: window::Settings {
            size: Size::new(1200.0, 800.0),
            resizable: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Clone)]
enum ForestState {
    ForestCreating(ForestBuilderPage),
    ForestCreated(Forest)
}
impl Default for ForestState {
    fn default() -> Self {
        ForestState::ForestCreating(ForestBuilderPage::default())
    }
}

#[derive(Debug, Clone)]
enum ForestMessage {
    AreaInputChanged(String),
    RainyCheckBoxToggled(bool),
    SunnyCheckBoxToggled(bool),
    GloomyCheckBoxToggled(bool),
    WithoutCheckBoxToggled(bool),
    CreateForest
}

impl Sandbox for ForestState {

    type Message = ForestMessage;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Forest builder".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match self {
            ForestState::ForestCreating(forest_builder_page) => {
                match message {
                    ForestMessage::RainyCheckBoxToggled(bool) => {
                        forest_builder_page.is_rainy_weather_added = bool;
                        forest_builder_page.is_sunny_weather_added = false;
                        forest_builder_page.is_gloomy_weather_added = false;
                        forest_builder_page.is_without_weather = false;
                    },

                    ForestMessage::SunnyCheckBoxToggled(bool) => {
                        forest_builder_page.is_sunny_weather_added = bool;
                        forest_builder_page.is_gloomy_weather_added = false;
                        forest_builder_page.is_without_weather = false;
                        forest_builder_page.is_rainy_weather_added = false;
                    },

                    ForestMessage::GloomyCheckBoxToggled(bool) => {
                        forest_builder_page.is_gloomy_weather_added = bool;
                        forest_builder_page.is_without_weather = false;
                        forest_builder_page.is_rainy_weather_added = false;
                        forest_builder_page.is_sunny_weather_added = false;
                    },

                    ForestMessage::WithoutCheckBoxToggled(bool) => {
                        forest_builder_page.is_without_weather = bool;
                        forest_builder_page.is_rainy_weather_added = false;
                        forest_builder_page.is_sunny_weather_added = false;
                        forest_builder_page.is_gloomy_weather_added = false;
                    },

                    ForestMessage::AreaInputChanged(area) => {
                        forest_builder_page.input_area = area;
                    },

                    ForestMessage::CreateForest => {
                        let mut forest = Forest::builder();



                        if !forest_builder_page.input_area.clone().is_empty() {
                            let area_usize = forest_builder_page.input_area.parse::<usize>().expect("Error with parsing Area");
                            forest.add_area(area_usize);
                        }


                        if forest_builder_page.is_rainy_weather_added == true {
                            forest.add_weather(Weather::Rainy);
                        }

                        if forest_builder_page.is_sunny_weather_added == true {
                            forest.add_weather(Weather::Sunny);
                        }

                        if forest_builder_page.is_gloomy_weather_added == true {
                            forest.add_weather(Weather::Gloomy);
                        }

                        *self = ForestCreated(forest.build());
                    }
                }
            }

            ForestState::ForestCreated(forest) => {

            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ForestState::ForestCreating(forest_builder_page) => {

                let forest_label = text("Forest Builder").size(25);

                let area_input = text_input("Add some area...", &forest_builder_page.input_area)
                    .width(250)
                    .padding(15)
                    .size(15)
                    .on_input(ForestMessage::AreaInputChanged);

                let rainy_checkbox = Checkbox::new("Add Rain", forest_builder_page.is_rainy_weather_added)
                    .on_toggle(ForestMessage::RainyCheckBoxToggled);

                let sunny_checkbox = Checkbox::new("Add Sun", forest_builder_page.is_sunny_weather_added)
                    .on_toggle(ForestMessage::SunnyCheckBoxToggled);

                let gloomy_checkbox = Checkbox::new("Add Gloom", forest_builder_page.is_gloomy_weather_added)
                    .on_toggle(ForestMessage::GloomyCheckBoxToggled);

                let without_checkbox = Checkbox::new("Without Weather", forest_builder_page.is_without_weather)
                    .on_toggle(ForestMessage::WithoutCheckBoxToggled);

                let create_btn = button("Create Forest")
                    .padding(20)
                    .style(Button::Primary)
                    .on_press(ForestMessage::CreateForest);

                container(column![
                    forest_label,
                    Space::with_height(15),
                    area_input,
                    row![rainy_checkbox, sunny_checkbox, gloomy_checkbox, without_checkbox].spacing(10),
                    create_btn
                ].spacing(20).align_items(Center)).width(Length::Fill).align_y(Vertical::Center).align_x(Horizontal::Center).into()
            }

            ForestState::ForestCreated(forest) => {

            }
        }
    }
}


