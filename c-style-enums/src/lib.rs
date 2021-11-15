
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
 Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    /// return the plural noun of this time unit
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// return the plural noun for this time unit
    fn singular(self) -> &'static str {
        self.plural.trim_end_mathces('s')
    }
}
