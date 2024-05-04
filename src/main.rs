use std::rc::Rc;
use iced::{alignment, Alignment, Element, Length, Sandbox, Settings, Size, window};
use iced::Alignment::Center;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{button, Checkbox, column, row, Container, container, text, text_input, Column, Space};
use crate::styles::ContainerStyle;
use crate::forest_components::{Color, Location, TreeKind, Weather};
use crate::model::{Forest};
use crate::forest_builder_page::{ForestBuilderPage};
use crate::tree_builder_page::{TreeBuilderPage};
use crate::ForestState::ForestCreated;

mod tree_builder_page;
mod forest_builder_page;
mod model;
mod forest_components;
mod styles;



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
    ForestCreated(Forest, TreeBuilderPage)
}
impl Default for ForestState {
    fn default() -> Self {
        ForestState::ForestCreating(ForestBuilderPage::default())
    }
}

#[derive(Debug, Clone)]
enum ForestMessage {
    //Forest_page events
    AreaInputChanged(String),
    RainyCheckBoxToggled(bool),
    SunnyCheckBoxToggled(bool),
    GloomyCheckBoxToggled(bool),
    WithoutCheckBoxToggled(bool),
    CreateForest,
    //Tree_page events
    KindNameInputChanged(String),
    ColorInputChanged(String),
    LocationXInputChanged(String),
    LocationYInputChanged(String),
    CreateKind,
    CreateTree
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

                        *self = ForestCreated(forest.build().clone(), TreeBuilderPage::default());
                    },

                    _ => {}
                }
            }

            ForestState::ForestCreated(forest, tree_builder_page) => {
                match message {
                    ForestMessage::KindNameInputChanged(name) => {
                        tree_builder_page.tree_kind_name = name;
                    }

                    ForestMessage::ColorInputChanged(color) => {
                        tree_builder_page.color_value = color;
                    }

                    ForestMessage::LocationXInputChanged(x) => {
                        tree_builder_page.location_x = x;
                    }

                    ForestMessage::LocationYInputChanged(y) => {
                        tree_builder_page.location_y = y;
                    }

                    ForestMessage::CreateKind => {

                    },

                    ForestMessage::CreateTree => {
                        let color = tree_builder_page.color_value.as_str();
                        let unwrapped_color = match color {
                            "Brown" => Color::Brown,
                            "Green" => Color::Green,
                            _ => Color::Unknown
                        };

                        forest.plant_tree(
                            Location::new(
                                tree_builder_page.location_x.parse::<i32>().unwrap(),
                                tree_builder_page.location_y.parse::<i32>().unwrap(),
                            ),
                            tree_builder_page.tree_kind_name.clone(),
                            unwrapped_color
                        )
                    },
                    _ => {}
                }
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

            ForestState::ForestCreated(forest, tree_builder_page) => {
                let amount_of_species = forest.tree_kinds.len();
                let amount_of_trees = forest.trees.len();
                let area_of_forest = forest.forest_area;
                let weather = match &forest.weather {
                    Some(weather_enum) => match weather_enum {
                        Weather::Gloomy => {
                            "Пасмурно".to_string()
                        },

                        Weather::Rainy => {
                            "Дождь".to_string()
                        },

                        Weather::Sunny => {
                            "Солнечно".to_string()
                        }
                    },
                    None => {
                        "Неизвестно".to_string()
                    }
                };

                let forest_text = text("Forest was created").size(30);

                let text_amount_of_species = text(format!("Количетсво видов: {amount_of_species}")).size(16);
                let text_amount_of_trees = text(format!("Количетсво деревьев: {amount_of_trees}")).size(16);
                let text_area_of_forest = text(format!("Площадь леса: {area_of_forest}")).size(16);
                let text_weather = text(format!("Погода: {weather}")).size(16);

                let forest_container = container(column![
                    forest_text,
                    Space::with_height(20),
                    text_amount_of_species,
                    text_amount_of_trees,
                    text_area_of_forest,
                    text_weather
                ].align_items(Alignment::Start)
                    .spacing(10))
                    .align_y(Vertical::Center)
                    .align_x(Horizontal::Center)
                    .padding(30)
                    .style(iced::theme::Container::Custom(Box::new(ContainerStyle)));


                let input_creator = |text, field| {
                    text_input(text, field)
                        .width(250)
                        .padding(15)
                        .size(15)
                };

                let tree_label = text("Tree Creator").size(35);

                let tree_inputs = column![
                    input_creator("Name of kind", &tree_builder_page.tree_kind_name).on_input(ForestMessage::KindNameInputChanged),
                    input_creator("Color of tree", &tree_builder_page.color_value).on_input(ForestMessage::ColorInputChanged),
                    input_creator("Location at x..", &tree_builder_page.location_x).on_input(ForestMessage::LocationXInputChanged),
                    input_creator("Location at y..", &tree_builder_page.location_y).on_input(ForestMessage::LocationYInputChanged)
                ].spacing(10);

                let create_tree_btn = button("Plant tree")
                    .padding(20)
                    .style(Button::Primary)
                    .on_press(ForestMessage::CreateTree);

                let tree_builder_cont = container(
                    row![tree_inputs,create_tree_btn].align_items(Alignment::Center).spacing(10)
                ).align_y(Vertical::Center).align_x(Horizontal::Center);


                container(
                    column![
                        Space::with_height(30),
                        forest_container,
                        Space::with_height(20),
                        tree_label,
                        tree_builder_cont
                    ].spacing(25).width(Length::Fill).align_items(Alignment::Center)

                ).center_x().center_y().into()
            }
        }
    }
}


