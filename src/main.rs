use iced::{
    alignment, button, executor, Alignment, Application, Button, Checkbox, Column, Command,
    Container, Element, Length, Settings, Subscription, Text, Color, Background, 
};
use iced_native::{window, Event, widget::container,widget::Row};

/*
    - Open a Window
    - Create Task struct only with task string as field
*/

fn main() -> iced::Result {
    App::run(Settings::default())
}



#[derive(Debug)]
struct Task {
    label: String,
}

impl Task {
    fn view(&mut self) -> Element<Message>{
        Row::new()
            .padding(5)
            .spacing(5)
            .push(
                Column::new().spacing(5)
                    .push(Text::new(self.label.as_str())
                        .size(10)
                        .vertical_alignment(alignment::Vertical::Center)
                    ),
            )
            .into()
    }
}

#[derive(Debug, Default)]
struct App {
    tasks: Option<Vec<Task>>,
}

impl App{
    fn render_tasks(tasks:&mut Vec<Task>) -> Element<Message>{
        let col = Column::new();
        let tasks : Element<_> = tasks.iter_mut()
            .fold(Column::new().spacing(10),|col,c|col.push(c.view())).into();
        col.push(tasks).into()
    }
}



impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        
        let tasks = vec![Task{
            label: "Open a window".to_owned(),
        }];

        let app = App{
            tasks: Some(tasks)
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        "The Todo List".to_owned()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        
        let tasks: Element<_> = match self.tasks {
            None => Column::new()
            .push(Text::new("loading...".to_owned()).size(15))
            .into(),
            Some(ref mut c) => App::render_tasks(c)
            
        }; 
        let col = Column::new()
            .max_width(600)
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(Text::new("Tasks").to_owned())
            .push(tasks);
        //let task_container = Container::new(tasks).align_y(iced::alignment::Vertical::Center);
        Container::new(col)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .into()
        
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Windowed
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    
}

struct RootContainer;

impl container::StyleSheet for RootContainer{
    fn style(&self) -> container::Style {
        iced::container::Style{
            background: Some(Background::from(Color::from_rgb(255.0, 1.0, 1.0))),
            ..Default::default()
        }
    }
}


#[derive(Debug, Default)]
struct Events {
    last: Vec<iced_native::Event>,
    enabled: bool,
    exit: button::State,
    should_exit: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
}

impl Application for Events {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        unimplemented!()
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    self.should_exit = true;
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;
            }
            Message::Exit => {
                self.should_exit = true;
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        let events = self
            .last
            .iter()
            .fold(Column::new().spacing(10), |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(40))
            });

        let toggle = Checkbox::new(self.enabled, "Listen to runtime events", Message::Toggled);

        let exit = Button::new(
            &mut self.exit,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(Length::Units(100))
        .padding(10)
        .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
