use chrono::{prelude::*, format::Fixed};
use iced::{
    alignment,
    widget::{column, container, row, text, Button, Column, Container, Row, Text, button, horizontal_space, text_input, scrollable},
    Application, Background, Color, Command, Element, Length, Settings, color, theme, Alignment, Theme, Font,
};

use iced_aw::{Card, Modal};
use iced_aw::{date_picker::Date as DateModal, DatePicker};

use iced::window;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    CalendarApp::run(Settings {
        window: window::Settings {
            size: (1200, 850),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum CalendarApp {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
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


#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    TitleInputChanged(String),
    CreateEvent,
    OpenModal,
    CloseModal,
    ChooseDate,
    SubmitDate(DateModal),
    CancelDate,
    EventMessage(usize, EventMessage),
    NextMonth,
    PrevMonth,
}


#[derive(Debug, Clone, Default, PartialEq)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    title: String,
    date: String,


    #[serde(skip)]
    state: EventState,
}

#[derive(Debug, Clone)]
pub enum EventState {
    Idle,
    Editing,
}

impl Default for EventState {
    fn default() -> Self {
        Self::Idle
    }
}


#[derive(Debug, Clone)]
pub enum EventMessage {
    Edit,
    TitleEdited(String),
    FinishEdition,
    Delete,
}

impl Event {
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("event-{i}"))
    }

    fn new(title: String, date: Date) -> Self {
        Event {
            title,
            date: date.format_date(),
            state: EventState::Idle,
        }
    }

    fn update(&mut self, message: EventMessage) {
        match message {
            EventMessage::Edit => {
                self.state = EventState::Editing;
            }
            EventMessage::TitleEdited(new_title) => {
                self.title = new_title;
            }
            EventMessage::FinishEdition => {
                if !self.title.is_empty() {
                    self.state = EventState::Idle;
                }
            }
            EventMessage::Delete => {}
           
        }
    }

    fn view(&self, i: usize) -> Element<EventMessage> {
        let title = self.title.as_str();

        match &self.state {
            EventState::Idle => {
                row![
                    text(title).size(16).width(Length::Fill),
                    button(edit_icon())
                        .on_press(EventMessage::Edit)
                        .padding(5)
                        .width(Length::Fill)
                        .style(theme::Button::Text)
                ]
                // .width(Length::Fill)
                .spacing(5)
                .align_items(Alignment::Center)
                .into()
            }
            EventState::Editing => {
                let text_input = text_input(
                    "Describe your event...",
                    &self.title,
                    EventMessage::TitleEdited,
                ).id(Self::text_input_id(i))
                .on_submit(EventMessage::FinishEdition)
                .padding(2);
                
                row![
                    text_input,
                    button(row![delete_icon()].spacing(2))
                        .on_press(EventMessage::Delete)
                        .padding(2)
                        .style(theme::Button::Destructive)
                ]
                .spacing(2)
                .align_items(Alignment::Center)
                .into()

            }
        }

    }
}


impl Application for CalendarApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CalendarApp, Command<Message>) {
        (
            CalendarApp::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }
    

    fn title(&self) -> String {
        let dirty = match self {
            CalendarApp::Loading => false,
            CalendarApp::Loaded(state) => state.dirty,
        };

        format!("Todos{} - Iced", if dirty { "***" } else { "" })
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match self {
            CalendarApp::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = CalendarApp::Loaded(State {
                            events: state.events,
                            date: Date::today(),
                            picked_date: DateModal::today(),
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = CalendarApp::Loaded(State{
                            date: Date::today(),
                            picked_date: DateModal::today(),
                            ..State::default()
                        });            
                    }
                    _ => {}
                }

                Command::none()
            }
            CalendarApp::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::NextMonth => {
                        if state.date.month == 12 {
                            state.date.month = 1;
                            state.date.year = state.date.year + 1;
                        } else {
                            state.date.month = state.date.month + 1;
                        }

                        Command::none()
                    }
                    Message::TitleInputChanged(value) => {
                        state.input_value = value;
                        
                        Command::none()
                    },
                    Message::CreateEvent => {
                        if !state.input_value.is_empty() {
                            state.events.push(
                                Event::new(
                                    state.input_value.clone(), 
                                    Date::from_ymd(state.picked_date.year, state.picked_date.month, state.picked_date.day)
                                )
                            );
                            state.input_value.clear();
                        }
                        //create event 
                        state.show_modal = false;

                        Command::none()
                    },
                    Message::OpenModal => {
                        state.show_modal = true;

                        Command::none()
                    },
                    Message::CloseModal => {
                        state.show_modal = false;

                        Command::none()
                    },
                    Message::ChooseDate => {
                        state.show_picker = true;
                        print!("check : {}", state.show_picker);
                        // state.show_modal = false;
                        Command::none()
                    },
                    Message::SubmitDate(picked_date) => {
                        state.picked_date = picked_date;
                        state.show_picker = false;

                        Command::none()
                    },
                    Message::CancelDate => {
                        state.show_picker = false;
                        Command::none()
                    },
                    Message::EventMessage(i, EventMessage::Delete) => {
                        state.events.remove(i);

                        Command::none()
                    },
                    Message::EventMessage(i, event_message) => {
                        if let Some(event) = state.events.get_mut(i) {
                            let should_focus = matches!(event_message, EventMessage::Edit);

                            event.update(event_message);

                            if should_focus {
                                let id = Event::text_input_id(i);

                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    Message::PrevMonth => {
                        if state.date.month == 1 {
                            state.date.month = 12;
                            state.date.year = state.date.year - 1;
                        } else {
                            state.date.month = state.date.month - 1;
                        }

                        Command::none()
                    },
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    },
                    Message::Loaded(_) => {
                        Command::none()
                    }
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            events: state.events.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
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
                        .style(theme::Container::Custom(Box::new(MyContainerStyle)));

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
                                .filter(|event| event.date == date2.format_date())
                                .collect::<Vec<_>>();

                            let events_day: Element<_> = scrollable(column(
                                event_this_day
                                .iter()
                                .enumerate()
                                .map(|(i, event)| {
                                    event.view(i).map(move |message| {
                                        Message::EventMessage(i, message)
                                    })
                                }).collect(),
                            )
                            .spacing(2)
                            ).style(theme::Scrollable::Custom(Box::new(MyScrollable)))
                            .into();

                            let day_element = if date2 == Date::today() {
                                text(date2.day.to_string())
                                .size(30)
                                .style(Color::from([44.0/255.0, 138.0/255.0, 252.0/255.0]))
                            } else {
                                text(date2.day.to_string())
                                .size(16)
                                .style(Color::from([0.6, 0.6, 0.6]))
                            };

                            let day_event = Container::new(column![
                                row![
                                    day_element
                                ]
                                .width(Length::Fill),
                                row![
                                    events_day
                                ]
                                .width(Length::Fill)
                            ])
                            .width(Length::Fill)
                            .height(Length::Fixed(120.0))

                            .center_x()
                            .padding(5)
                            .style(theme::Container::Custom(Box::new(MyContainerStyle)));
                            week = week.push(day_event);
                        } else {
                            week = week.push(Container::new("")
                            .width(Length::Fill)
                            .height(Length::Fixed(120.0))
                            .center_x()
                            .padding(5)
                            .style(theme::Container::Custom(Box::new(MyContainerStyle))));
                        }
                    }

                    day_current_month = day_current_month.push(week);
                }

                let content= column![header, weekday, day_current_month];

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
                                .id(INPUT_ID.clone())
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum Filter {
//     Holiday,
//     Todo,
//     Reminder,
// }

// impl Default for Filter {
//     fn default() -> Self {
//         Filter::All
//     }
// }

// impl Filter {
//     fn matches(&self, task: &Task) -> bool {
//         match self {
//             Filter::All => true,
//             Filter::Active => !task.completed,
//             Filter::Completed => task.completed,
//         }
//     }
// }


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

struct MyScrollable;

impl scrollable::StyleSheet for MyScrollable {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar { 
            background: Color::TRANSPARENT.into(), 
            border_radius: 2.0, 
            border_width: 0.0, 
            border_color: Color::TRANSPARENT, 
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar { 
            background: Color::TRANSPARENT.into(), 
            border_radius: 2.0, 
            border_width: 0.0, 
            border_color: Color::TRANSPARENT, 
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../date_picker/fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(16)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(16)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    events: Vec<Event>,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}


#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "CalendarApp")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("calendar.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

