
use chrono::prelude::*;
use iced::{
    alignment,
    widget::{column, container, row, text, Button, Column, Container, Row, Text, button, horizontal_space},
    Application, Background, Color, Command, Element, Length, Sandbox, Settings, color, theme, Alignment,
};
use iced_graphics::Primitive;
use iced_native::{Layout, renderer::BorderRadius, widget::scrollable::style};

use iced_aw::{Card, Modal};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    pub fn today() -> Self {
        let today: DateTime<Local> = Local::now();
        Self {
            year: today.year(),
            month: today.month(),
            day: today.day(),
        }
    }

    /// Check leap year
    fn check_leap_year(&self, year: i32) -> bool {
        let mod4 = year % 4 == 0;
        let mod100 = year % 100 == 0;
        let mod400 = year % 400 == 0;

        mod4 && (!mod100 || mod400)
    }

    /// Check how many days of the current month
    fn number_days_month(&self, month: u32, year: i32) -> u32 {
        match month {
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.check_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        }
    }

    /// Check how many days of the last month
    fn number_days_pred_month(&self, month: u32, year: i32) -> u32 {
        if month == 1 {
            self.number_days_month(12, year - 1)
        } else {
            self.number_days_month(month - 1, year)
        }
    }

    /// Check how many days of the next month
    fn number_days_next_month(&self, month: u32, year: i32) -> u32 {
        if month == 12 {
            self.number_days_month(1, year + 1)
        } else {
            self.number_days_month(month + 1, year)
        }
    }

    /// format date 2022-12-05
    pub fn format_date(&self) -> String {
        let dt = Utc.ymd(self.year, self.month, self.day);
        dt.format("%Y-%m-%d").to_string()
    }

    pub const fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

struct State {
    date: Date,
    events: Vec<Event>,

    saving: bool,
    dirty: bool,
}


impl State {}

#[derive(Debug, Clone)]
struct SavedState {
    events: Vec<Event>,
}
enum CalendarApp {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
struct Event {
    title: String,
    date: Date,
}

#[derive(Debug, Clone)]
enum Message {
    TitleInputChanged(String),
    CreateEvent,
    EventMessage(usize, EventMessage),
    NextMonth,
    PrevMonth,
}

#[derive(Debug, Clone)]
pub enum EventMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}


impl Sandbox for CalendarApp {
    type Message = Message;

    fn new() -> Self {
        CalendarApp::Loaded(State {
            date: Date::today(),
            events: vec![],
            saving: true,
            dirty: false,
        })
    }

    fn title(&self) -> String {
        String::from("Calendar App")
    }

    fn update(&mut self, message: Self::Message) {
        match self {
            CalendarApp::Loading => {

            }
            CalendarApp::Loaded(state) => {
                match message {
                    Message::NextMonth => {
                        if state.date.month == 12 {
                            state.date.month = 1;
                            state.date.year = state.date.year + 1;
                        } else {
                            state.date.month = state.date.month + 1;
                        }
                    }
                    Message::TitleInputChanged(_) => todo!(),
                    Message::CreateEvent => {

                    },
                    Message::EventMessage(_, _) => todo!(),
                    Message::PrevMonth => {
                        if state.date.month == 1 {
                            state.date.month = 12;
                            state.date.year = state.date.year - 1;
                        } else {
                            state.date.month = state.date.month - 1;
                        }
                    },
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            CalendarApp::Loading => loading_message(),
            CalendarApp::Loaded(State { date, events, .. }) => {
                let dt = Utc.ymd(date.year, date.month, date.day);

                let month_start_day = Utc.ymd(date.year, date.month, 1).weekday().num_days_from_sunday();

                let weekdays = dt.weekday().num_days_from_monday();

                let days = date.number_days_month(date.month, date.year);

                let pred_month_days = date.number_days_pred_month(date.month, date.year);

                let next_month_days = date.number_days_next_month(date.month, date.year);

                let months_text = vec!["January","February","March","April","May","June","July","August","September","October","November","December",];

                let days_text = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

                let month_text: Text = text(months_text[(date.month as usize) - 1])
                    .size(32)
                    .style(Color::from([0.6, 0.6, 0.6]))
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center);

                let year_text: Text = text(date.year.to_string())
                    .size(32)
                    .style(Color::from([0.6, 0.6, 0.6]))
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center);

                let header = view_controls(month_text, year_text);

                // Create a header for the weekdays name
                let mut weekday = Row::new();

                for day in days_text.iter() {
                    // Use the Text widget to display the day
                    let text = Text::new(day.to_string())
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .style(Color::from([0.5, 0.5, 0.5]));
                    // Wrap the Text widget in a Container with a background color and padding
                    let container = Container::new(text)
                        .width(Length::Fill)
                        .height(Length::Units(30))
                        .center_x()
                        .center_y()
                        .padding(5)
                        .style(iced::theme::Container::Custom(Box::new(MyContainerStyle)));

                    weekday = weekday.push(container);
                }

                for day_event in 1..days {

                } 

                let mut day_current_month = Column::new();

                let mut day_count = 0;

                for _ in 0..6 {
                    let mut week = Row::new();

                    for weekday_num in 0..7 {
                        if (month_start_day == weekday_num || day_count >= 1) && day_count < days
                        {
                            day_count += 1;

                            let date2 = Date::from_ymd(date.year, date.month, day_count);
                            
                            let event_this_day = events.iter()
                                .filter(|event| event.date == date2)
                                .collect::<Vec<_>>();

                            let vec: Vec<Event> = event_this_day.iter().map(|event| event.clone().clone()).collect();
                            // let day_event = DayEvent::new(date2, vec);
                            let day_event = Container::new(column![
                                row![
                                    text(date2.day.to_string())
                                    .size(16)
                                    .style(Color::from([0.6, 0.6, 0.6]))
                                ]
                                .width(Length::Fill),
                                row![
                                    
                                ]
                            ])
                            .width(Length::Fill)
                            .height(Length::Units(120))
                            .center_x()
                            // .center_y()
                            .padding(5)
                            .style(iced::theme::Container::Custom(Box::new(MyContainerStyle)));
                            week = week.push(day_event);
                        } else {
                            week = week.push(Container::new("")
                            .width(Length::Fill)
                            .height(Length::Units(120))
                            .center_x()
                            // .center_y()
                            .padding(5)
                            .style(iced::theme::Container::Custom(Box::new(MyContainerStyle))));
                        }
                    }

                    day_current_month = day_current_month.push(week);
                }

                let content = Column::new()
                .push(header)
                .push(weekday)
                .push(day_current_month);

                container(content).center_x().into()
            }
        }
    }
}

fn view_controls<'a>(month_text: Text<'a>, year_text: Text<'a>) -> Element<'a, Message> {
    column![
        row![
            // horizontal_space(Length::Fill),
            row![
                month_text,
                year_text,
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center),
            horizontal_space(Length::Fill),
            row![
                button("Create new event")
                    .on_press(Message::CreateEvent)
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center),
            horizontal_space(Length::Fill)
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center),

        row![
            button("prev month")
                .style(theme::Button::Text)
                .on_press(Message::PrevMonth),
            horizontal_space(Length::Fill),
            button("next month")
                .style(theme::Button::Text)
                .on_press(Message::NextMonth),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
    ].into()
}

#[derive(Default)]
struct ModalEvent {
    show_modal: bool,
    last_message: Option<MessageModal>
}

#[derive(Clone, Debug)]
enum MessageModal {
    OpenModal,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
}

impl ModalEvent {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, message: MessageModal) {
        match message {
            MessageModal::OpenModal => self.show_modal = true,
            MessageModal::CloseModal => self.show_modal = false,
            MessageModal::CancelButtonPressed => self.show_modal = false,
            MessageModal::OkButtonPressed => self.show_modal = false,
        }
        self.last_message = Some(message)
    }

    // fn view(&self) -> Element<MessageModal> {
    //     let content = Container::new(
    //         Row::new()
    //             .spacing(10)
    //             .align_items(Alignment::Center)
    //             .push(Button::new(Text::new("Open modal!")).on_press(MessageModal::OpenModal))
    //     );

    //     let card = Card::new(
    //         Text::new("Create a new event"),
    //         text_input()
    //     )

    //     let modal = Modal::new(self.show_modal, content, || {
    //         Card::new(
    //             Text::new("My modal"),
    //             Text::new("This is a modal!"), //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
    //         )
    //         .foot(
    //             Row::new()
    //                 .spacing(10)
    //                 .padding(5)
    //                 .width(Length::Fill)
    //                 .push(
    //                     Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
    //                         .width(Length::Fill)
    //                         .on_press(Message::CancelButtonPressed),
    //                 )
    //                 .push(
    //                     Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
    //                         .width(Length::Fill)
    //                         .on_press(Message::OkButtonPressed),
    //                 ),
    //         )
    //         .max_width(300.0)
    //         //.width(Length::Shrink)
    //         .on_close(Message::CloseModal)
    //         .into()
    //     })
    //     .backdrop(Message::CloseModal)
    //     .on_esc(Message::CloseModal);
    // }
    
}

pub fn main() {
    <CalendarApp as Sandbox>::run(Settings::default());
}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}


struct MyContainerStyle;

impl container::StyleSheet for MyContainerStyle {
  type Style = iced::Theme;
  fn appearance(&self, style: &iced::Theme) -> container::Appearance {
    container::Appearance {
        border_width: 1.0,
        border_radius: 1.0,
        border_color: Color::BLACK,
        ..Default::default()
    }
  }
}









