use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};



#[derive(Debug)]
pub enum Message {
    Task(State),
}

#[derive(Debug, Default)]
pub struct State {
    done: bool,
}


pub struct HomePage {

}

impl Application for HomePage {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        String::from("The home page")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Task(State) => {
                match State.done {
                    true => {}
                    false => {}
                }
            }
            _ => ()
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
    
} 
