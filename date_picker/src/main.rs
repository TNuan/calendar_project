
use std::sync::Arc;

use chrono::prelude::*;
use iced::{
    alignment,
    widget::{column, container, row, text, Button, Column, Container, Row, Text, button, horizontal_space, text_input},
    Application, Background, Color, Command, Element, Length, Sandbox, Settings, color, theme, Alignment,
};
use iced_native::{Layout, renderer::BorderRadius, widget::scrollable::style};

use iced_aw::{Card, Modal};
use iced_aw::{date_picker::Date as DateModal, DatePicker};

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
    show_modal: bool,
    show_picker: bool,
    saving: bool,
    dirty: bool,
    input_value: String,
    picked_date: DateModal,
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
    OpenModal,
    CloseModal,
    // MessageDatepicker(MessageDatepicker),
    ChooseDate,
    SubmitDate(DateModal),
    CancelDate,
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
            show_modal: false,
            show_picker: false,
            saving: true,
            dirty: false,
            input_value: String::from(""),
            picked_date: DateModal::today(),
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
                    Message::TitleInputChanged(value) => {
                        state.input_value = value;
                    },
                    Message::CreateEvent => {
                        if !state.input_value.is_empty() {
                            state.input_value.clear();
                        }
                        //create event 
                        state.show_modal = false;
                    },
                    Message::OpenModal => {
                        state.show_modal = true;
                    },
                    Message::CloseModal => {
                        state.show_modal = false;
                    },
                    Message::ChooseDate => {
                        state.show_picker = true;
                        print!("check : {}", state.show_picker);
                        // state.show_modal = false;
                    },
                    Message::SubmitDate(picked_date) => {
                        state.picked_date = picked_date;
                        state.show_picker = false;
                    },
                    Message::CancelDate => {
                        state.show_picker = false;
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
            CalendarApp::Loaded(State 
                { 
                    date, 
                    events, 
                    show_modal,
                    show_picker,  
                    input_value,
                    picked_date,  
                    .. 
                }
            ) => {
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

                let header = view_controls(month_text, year_text, *show_modal, *show_picker, input_value.to_string(), *picked_date );

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
                        .height(Length::Fixed(30.0))
                        .center_x()
                        .center_y()
                        .padding(5)
                        .style(iced::theme::Container::Custom(Box::new(MyContainerStyle)));

                    weekday = weekday.push(container);
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
                            .height(Length::Fixed(120.0))
                            .center_x()
                            // .center_y()
                            .padding(5)
                            .style(iced::theme::Container::Custom(Box::new(MyContainerStyle)));
                            week = week.push(day_event);
                        } else {
                            week = week.push(Container::new("")
                            .width(Length::Fill)
                            .height(Length::Fixed(120.0))
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

fn view_controls<'a>(month_text: Text<'a>, year_text: Text<'a>, show_modal: bool, show_picker: bool, input_value: String, picked_date: DateModal) -> Element<'a, Message> {
    let create_event_btn = Container::new(
        Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Button::new(Text::new("Create event")).on_press(Message::OpenModal)));

    column![
        row![
            // horizontal_space(Length::Fill),
            row![
                Modal::new(show_modal, create_event_btn, move ||  {
                        Card::new(
                            Text::new("Create a new event"),
                            column![
                                text_input(
                                    "What needs to be done?",
                                    &input_value,
                                    Message::TitleInputChanged,
                                )
                                .on_submit(Message::CreateEvent),
                                row![
                                    DatePicker::new(
                                        show_picker,
                                        picked_date,
                                        button("Set Date").style(theme::Button::Text).on_press(Message::ChooseDate),
                                        Message::CancelDate,
                                        Message::SubmitDate,
                                    ),
                                    text(format!("Date: {}", picked_date))
                                ]
                                .align_items(alignment::Alignment::Center)
                                .spacing(10)
                            ]
                            .spacing(10)
                        )
                        .foot(
                            Row::new()
                                .spacing(10)
                                .padding(5)
                                .width(Length::Fill)
                                .push(
                                    Button::new(Text::new("Cancel").horizontal_alignment(alignment::Horizontal::Center))
                                        .width(Length::Fill)
                                        .on_press(Message::CloseModal),
                                )
                                .push(
                                    Button::new(Text::new("Ok").horizontal_alignment(alignment::Horizontal::Center))
                                        .width(Length::Fill)
                                        .on_press(Message::CreateEvent),
                                )
                        )
                        .max_width(300.0)
                        .on_close(Message::CloseModal)
                        .into()
                    }
                )
                .backdrop(Message::CloseModal)
                .on_esc(Message::CloseModal),
                // .into()

                // DatePicker::new(
                //     show_picker,
                //     picked_date,
                //     button("Set Date").style(theme::Button::Text).on_press(Message::ChooseDate),
                //     Message::CancelDate,
                //     Message::SubmitDate,
                // ),
                // text(format!("Date: {}", picked_date))
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center),
            horizontal_space(Length::Fill),
            row![
                month_text,
                year_text,
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center),
            horizontal_space(Length::Fill),
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

pub fn main() {
    <CalendarApp as Sandbox>::run(Settings::default());
}


////////////////////////////////////////////////////////////////


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









