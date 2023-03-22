use core::slice::Iter;

enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    fn iter() -> Iter<'static, Month> {
        static MONTHS: [Month; 12] = [
            Month::Jan,
            Month::Feb,
            Month::Mar,
            Month::Apr,
            Month::May,
            Month::Jun,
            Month::Jul,
            Month::Aug,
            Month::Sep,
            Month::Oct,
            Month::Nov,
            Month::Dec,
        ];
        MONTHS.iter()
    }
}

enum Week {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl Iterator for Week {
    type Item = Week;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Week::Sun => {
                *self = Week::Mon;
                Some(Week::Mon)
            }
            Week::Mon => {
                *self = Week::Tue;
                Some(Week::Tue)
            }
            Week::Tue => {
                *self = Week::Wed;
                Some(Week::Wed)
            }
            Week::Wed => {
                *self = Week::Thu;
                Some(Week::Thu)
            }
            Week::Thu => {
                *self = Week::Fri;
                Some(Week::Fri)
            }
            Week::Fri => {
                *self = Week::Sat;
                Some(Week::Sat)
            }
            Week::Sat => {
                *self = Week::Sun;
                Some(Week::Sun)
            }
        }
    }
}

fn main() {
    let mut year = 1900;
    let mut first = Week::Mon; // 1 Jan 1900 was a Monday
    let mut count = 0;

    // 1900
    for m in Month::iter() {
        first.nth(month_to_step(m, year));
    }

    year += 1;
    // This should be Tuesday
    // println!("1 Jan {year} is {first:?}");

    while year < 2001 {
        for m in Month::iter() {
            if let Some(Week::Sun) = first.nth(month_to_step(m, year)) {
                count += 1;
            }
        }
        year += 1;
    }

    println!("Problem 019: {count}")
}

fn month_to_step(m: &Month, y: u32) -> usize {
    match m {
        Month::Jan => (31 - 1) % 7,
        Month::Feb => {
            if y % 400 == 0 {
                (29 - 1) % 7
            } else if y % 100 == 0 {
                (28 - 1) % 7
            } else if y % 4 == 0 {
                (29 - 1) % 7
            } else {
                (28 - 1) % 7
            }
        }
        Month::Mar => (31 - 1) % 7,
        Month::Apr => (30 - 1) % 7,
        Month::May => (31 - 1) % 7,
        Month::Jun => (30 - 1) % 7,
        Month::Jul => (31 - 1) % 7,
        Month::Aug => (31 - 1) % 7,
        Month::Sep => (30 - 1) % 7,
        Month::Oct => (31 - 1) % 7,
        Month::Nov => (30 - 1) % 7,
        Month::Dec => (31 - 1) % 7,
    }
}
