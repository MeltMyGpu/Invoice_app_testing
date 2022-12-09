use std::default;

use iced::{executor, Length};
use iced::widget::{Container, Text};
use iced::{Application, Command, Element, Settings, Theme};
use iced::Sandbox;
use crate::counter::Message;





#[derive(Default)]
pub struct HomePage {
}

impl Sandbox for HomePage {

    fn new() -> Self{
        HomePage {}
    }

    fn view(&self) -> Element<Message> {
        Container::new(Text::new("hello there..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    type Message = Message;

    fn title(&self) -> String {
        String::from("Home Page")
    }

    fn update(&mut self, message: Self::Message) {
       match message {
           _ => {}
       }
    }

}

 
