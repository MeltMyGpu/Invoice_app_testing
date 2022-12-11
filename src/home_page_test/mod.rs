

use iced::{ Length};
use iced::widget::{Container, Text};
use iced::{Element};
use crate::counter::Message;





#[derive(Default)]
pub struct HomePage {
}

impl HomePage {

    pub fn new() -> Self{
        HomePage {}
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(Text::new("hello there..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}

 
