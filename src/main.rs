

use counter::{Message, Counter};
use home_page_test::HomePage;
use iced::{Sandbox, Element, Settings};

pub mod counter;
pub mod home_page_test;

#[derive(Debug,Clone,Copy)]
pub enum Views {
    Counter,
    HomePage
}

#[derive(Debug)]
enum AppMessage {

}


struct AppManager {
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

fn main() {
    AppManager::run(Settings::default());
}
