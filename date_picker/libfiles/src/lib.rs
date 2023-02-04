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
        #[must_use]
        pub fn today() -> Self {
            let today = Local::now().naive_local().date();
            today.into()
        }
    }
}
