// Iced
use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced::widget::{column, button, text, container, scrollable};

// State
#[derive(Default)]
pub struct Counter {
    // counter value
    value : i32,
}

//messages (define all possible user interaction of our counter)
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}




impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Testinf counter")
    }
    
    fn update(&mut self, message: Message){
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }


    // View logic - dispalys state as widgets that may produce messages on interaction

    fn view(&self) -> Element<Message>{

        // use columb, simple vertical layout
        let content = column![
            // the increment button, we tell it to produce an 'incrementPressed' message
            button("+").on_press(Message::IncrementPressed),

            //show the value of the counter here
            text(self.value).size(50),

            // the dec button 
            button("-").on_press(Message::DecrementPressed),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }

    // Update logic 

}


pub fn run_execute() {
    Counter::run(Settings::default());
}