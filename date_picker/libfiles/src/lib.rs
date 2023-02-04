mod date {
    use std::fmt::Display;

    use chrono::Local;
    use chrono::{Datelike, Duration, NaiveDate};

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Date {
        pub year: i32,
        pub month: u32,
        pub day: u32,
    }

    impl Date {
        /// Tao ngay moi tu thoi gian hien tai
        #[must_use]
        pub fn today() -> Self {
            let today = Local::now().naive_local().date();
            today.into()
        }

        /// Creates a new date.
        #[must_use]
        pub const fn from_ymd(year: i32, month: u32, day: u32) -> Self {
            Self { year, month, day }
        }
    }

    impl Display for Date {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl From<Date> for NaiveDate {
        fn from(date: Date) -> Self {
            Self::from_ymd(date.year, date.month, date.day)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl From<NaiveDate> for Date {
        fn from(date: NaiveDate) -> Self {
            Self::from_ymd(date.year(), date.month(), date.day())
        }
    }

    /// Creates a date with the previous month based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn pred_month(date: NaiveDate) -> NaiveDate {
        let (year, month) = if date.month() == 1 {
            (date.year() - 1, 12)
        } else {
            (date.year(), date.month() - 1)
        };

        let day = date.day().min(num_days_of_month(year, month));

        NaiveDate::from_ymd(year, month, day)
    }

    /// Creates a date with the next month based on given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn succ_month(date: NaiveDate) -> NaiveDate {
        let (year, month) = if date.month() == 12 {
            (date.year() + 1, 1)
        } else {
            (date.year(), date.month() + 1)
        };

        let day = date.day().min(num_days_of_month(year, month));

        NaiveDate::from_ymd(year, month, day)
    }

    /// Creates a date with the previous year based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn pred_year(date: NaiveDate) -> NaiveDate {
        let year = date.year() - 1;
        let day = date.day().min(num_days_of_month(year, date.month()));

        NaiveDate::from_ymd(year, date.month(), day)
    }

    /// Creates a date with the next year based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn succ_year(date: NaiveDate) -> NaiveDate {
        let year = date.year() + 1;
        let day = date.day().min(num_days_of_month(year, date.month()));

        NaiveDate::from_ymd(year, date.month(), day)
    }

    /// Calculates a date with the previous week based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn pred_week(date: NaiveDate) -> NaiveDate {
        date - Duration::days(7)
    }

    /// Calculates a date with the next week based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn succ_week(date: NaiveDate) -> NaiveDate {
        date + Duration::days(7)
    }

    /// Calculates a date with the previous day based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn pred_day(date: NaiveDate) -> NaiveDate {
        date - Duration::days(1)
    }

    /// Calculates a date with the next day based on the given date.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn succ_day(date: NaiveDate) -> NaiveDate {
        date + Duration::days(1)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, PartialEq, Eq)]

    pub enum IsInMonth {
        // Ngay cua thang truoc
        Previous,
        // Ngay cua thang nay
        Same,
        // Ngay cua thang sau
        Next,
    }

    /// Tinh toan vi tri cua ngay tren datepicker dua vao thang va nam    
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn position_to_day(x: usize, y: usize, year: i32, month: u32) -> (usize, IsInMonth) {
        let (x, y) = (x as isize, y as isize);
        let first_day = NaiveDate::from_ymd(year, month, 1);
        let day_of_week = first_day.weekday().num_days_from_monday() as isize;
        let day_of_week = if day_of_week == 0 { 7 } else { day_of_week };

        let day = (x + 7 * y) + 1 - day_of_week;

        if day < 1 {
            let last_month = first_day.pred();
            (
                (num_days_of_month(last_month.year(), last_month.month()) as isize + day) as usize,
                IsInMonth::Previous,
            )
        } else if day > num_days_of_month(year, month) as isize {
            (
                (day - num_days_of_month(year, month) as isize) as usize,
                IsInMonth::Next,
            )
        } else {
            (day as usize, IsInMonth::Same)
        }
    }

    /// Tinh nam nhuan
    #[cfg(not(target_arch = "wasm32"))]
    const fn is_leap_year(year: i32) -> bool {
        let mod4 = year % 4 == 0;
        let mod100 = year % 100 == 0;
        let mod400 = year % 400 == 0;

        mod4 && (!mod100 || mod400)
    }

    /// Tinh so ngay cua thang trong nam.
    #[cfg(not(target_arch = "wasm32"))]
    const fn num_days_of_month(year: i32, month: u32) -> u32 {
        match month {
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        }
    }
}

mod style_date_picker {
    /**
     *
     */
    use iced_native::{Background, Color};
    use iced_style::Theme;

    #[derive(Clone, Copy, Debug)]
    pub struct Appearance {
        pub background: Background,
        pub border_radius: f32,
        pub border_width: f32,
        pub border_color: Color,
        pub text_color: Color,
        pub text_attenuated_color: Color,
        pub day_background: Background,
    }

    pub trait StyleSheet {
        type Style: std::default::Default + Copy;

        fn active(&self, style: Self::Style) -> Appearance;
        fn selected(&self, style: Self::Style) -> Appearance;
        fn hovered(&self, style: Self::Style) -> Appearance;
        fn focused(&self, style: Self::Style) -> Appearance;
    }

    pub struct Default;

    impl StyleSheet for Theme {
        type Style = Default;

        fn active(&self, _style: Self::Style) -> Appearance {
            let palette = self.extended_palette();
            let foreground = self.palette();
    
            Appearance {
                background: palette.background.base.color.into(),
                border_radius: 15.0,
                border_width: 1.0,
                border_color: foreground.text,
                text_color: foreground.text,
                text_attenuated_color: Color {
                    a: foreground.text.a * 0.5,
                    ..foreground.text
                },
                day_background: palette.background.base.color.into(),
            }
        }
        
        fn selected(&self, style: Self::Style) -> Appearance {
            let palette = self.extended_palette();
    
            Appearance {
                day_background: palette.primary.strong.color.into(),
                text_color: palette.primary.strong.text,
                ..self.active(style)
            }
        }

        fn hovered(&self, style: Self::Style) -> Appearance {
            let palette = self.extended_palette();
    
            Appearance {
                day_background: palette.primary.weak.color.into(),
                text_color: palette.primary.weak.text,
                ..self.active(style)
            }
        }

        fn focused(&self, style: Self::Style) -> Appearance {
            Appearance {
                border_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..self.active(style)
            }
        }
    }
}
