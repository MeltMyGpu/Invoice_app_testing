
use crate::counter::{Message, Counter};
use crate::home_page_test::HomePage;
use iced::{Sandbox, Element, Settings};



#[derive(Debug,Clone,Copy)]
pub enum Views {
    Counter,
    HomePage
}

#[derive(Debug)]
enum AppMessage {

}


pub struct AppManager {
    current_view : Views,
    home_page : HomePage,
    counter : Counter
}

impl Sandbox for AppManager {
    type Message = Message;
    
    fn new() -> Self {
        AppManager { 
            current_view: (Views::Counter) ,
            home_page: HomePage::new(),
            counter: Counter::new()
        }
    }


    fn title(&self) -> String {
        match self.current_view {
            Views::Counter => self.counter.title(),
            Views::HomePage => self.home_page.title()
        } 
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangePage(view) => {
                self.current_view = view;
            }
            _ => { self.counter.update(message) }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.current_view {
            Views::Counter => self.counter.view(),
            Views::HomePage => self.home_page.view()
        }
    }
  
}

pub fn run() {
    AppManager::run(Settings::default());
}