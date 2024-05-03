use std::fmt::format;
use iced::{alignment, Alignment, Element, Length, Sandbox, Settings, Size, window};
use iced::Alignment::Center;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{button, Checkbox, column, Container, container, text, text_input, Column, Space};
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
    WeatherCheckBoxToggled(bool),
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
                    ForestMessage::WeatherCheckBoxToggled(flag) => {
                        forest_builder_page.is_weather_cb_checked = flag
                    },

                    ForestMessage::AreaInputChanged(area) => {
                        forest_builder_page.input_area = area;
                    },

                    ForestMessage::CreateForest => {
                        let mut forest = Forest::builder();

                        if forest_builder_page.is_weather_cb_checked == true {
                            forest.add_weather(Weather::Rainy);
                        }

                        if !forest_builder_page.input_area.clone().is_empty() {
                            let area_usize = forest_builder_page.input_area.parse::<usize>().expect("Error with parsing Area");
                            forest.add_area(area_usize);
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

                let weather_checkbox = Checkbox::new("Add Weather?", forest_builder_page.is_weather_cb_checked)
                    .on_toggle(ForestMessage::WeatherCheckBoxToggled);

                let create_btn = button("Create Forest")
                    .padding(20)
                    .style(Button::Primary)
                    .on_press(ForestMessage::CreateForest);

                container(column![
                    forest_label,
                    Space::with_height(15),
                    area_input,
                    weather_checkbox,
                    create_btn
                ].spacing(20).align_items(Center)).width(Length::Fill).align_y(Vertical::Center).align_x(Horizontal::Center).into()
            }

            ForestState::ForestCreated(forest) => {
                todo!()
            }
        }
    }
}


