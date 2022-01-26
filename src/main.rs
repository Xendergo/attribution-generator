mod data;
mod styling;

use std::{collections::HashSet, path::PathBuf};

use data::{get_directory_data, get_path};
use iced::*;

use crate::data::write_path;

fn main() -> iced::Result {
    <App as Sandbox>::run(Settings::default())
}

struct App {
    location: Option<String>,
    attributions_ticked: HashSet<String>,
    attribution_options: Vec<(String, String)>,
    location_state: text_input::State,
    scroll_state: scrollable::State,
}

#[derive(Debug, Clone)]
enum Message {
    LocationChanged(String),
    TickAttribution(usize),
    UntickAttribution(usize),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let maybe_path = get_path();

        App {
            attribution_options: match &maybe_path {
                Some(path) => match get_directory_data(PathBuf::from(path)) {
                    Ok(options) => options,
                    Err(_) => Vec::new(),
                },
                None => Vec::new(),
            },
            location: maybe_path,
            attributions_ticked: HashSet::new(),
            location_state: text_input::State::default(),
            scroll_state: scrollable::State::default(),
        }
    }

    fn title(&self) -> String {
        "Attribution generator".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LocationChanged(v) => {
                let path = PathBuf::from(&v);

                self.location = Some(v.clone());

                if let Ok(options) = get_directory_data(path) {
                    self.attribution_options = options;

                    write_path(v).ok();
                }
            }

            Message::TickAttribution(index) => {
                self.attributions_ticked
                    .insert(self.attribution_options[index].0.clone());
            }

            Message::UntickAttribution(index) => {
                self.attributions_ticked
                    .remove(&self.attribution_options[index].0);
            }
        }
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Scrollable::new(&mut self.scroll_state)
            .push(
                Column::with_children(vec![
                    TextInput::new(
                        &mut self.location_state,
                        "Location of attribution files",
                        match &self.location {
                            Some(v) => v,
                            None => "",
                        },
                        Message::LocationChanged,
                    )
                    .padding(styling::PADDING)
                    .style(styling::TextInput())
                    .into(),
                    Self::attribution_list(&self.attribution_options, &self.attributions_ticked),
                ])
                .align_items(Align::Center)
                .spacing(styling::PADDING)
                .padding(styling::PADDING),
            )
            .into()
    }
}

impl App {
    fn attribution_list<'a>(
        attribution_options: &Vec<(String, String)>,
        attributions_ticked: &HashSet<String>,
    ) -> Element<'a, Message> {
        Column::with_children(
            attribution_options
                .iter()
                .enumerate()
                .map(|(i, (name, _))| Self::attribution_row(attributions_ticked, name, i))
                .collect(),
        )
        .padding(styling::PADDING)
        .spacing(styling::PADDING)
        .into()
    }

    fn attribution_row<'a>(
        attributions_ticked: &HashSet<String>,
        name: &str,
        i: usize,
    ) -> Element<'a, Message> {
        Row::with_children(vec![
            Checkbox::new(attributions_ticked.contains(name), "", move |ticked| {
                if ticked {
                    Message::TickAttribution(i)
                } else {
                    Message::UntickAttribution(i)
                }
            })
            .into(),
            Text::new(name)
                .color(if i % 2 == 0 {
                    Color::from_rgb8(255, 255, 255)
                } else {
                    Color::from_rgb8(184, 184, 184)
                })
                .into(),
        ])
        .into()
    }
}
