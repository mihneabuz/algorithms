use std::cmp::Ordering;
use std::cmp;

struct Interval {
    start: i32,
    end: i32,
    bookings: i32,
}

impl Interval {
    fn new(start: i32, end: i32, bookings: i32) -> Self {
        Self {
            start,
            end,
            bookings,
        }
    }
}

struct MyCalendarThree {
    intervals: Vec<Interval>,
    max: i32
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            intervals: vec![Interval::new(0, 10i32.pow(9), 0)],
            max: 0,
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.combine(0, start, end);
        self.max
    }

    fn combine(&mut self, mut i: usize, start: i32, end: i32) {
        if start == end {
            return;
        }

        while self.intervals[i].end <= start {
            i += 1;
        }

        let interval = &self.intervals[i];
        match (interval.start.cmp(&end), interval.start.cmp(&start), interval.end.cmp(&end)) {
            (Ordering::Greater, _, _) => {
                self.intervals.insert(i, Interval::new(start, end, 1));
            }

            (_, Ordering::Equal, Ordering::Equal) => {
                self.intervals[i].bookings += 1;
                self.max = cmp::max(self.max, self.intervals[i].bookings);
            }


            (_, Ordering::Less, Ordering::Greater) => {
                self.intervals.insert(i + 1, Interval::new(start, interval.end, interval.bookings));
                self.intervals[i].end = start;
                self.combine(i + 1, start, end);
            }

            (_, Ordering::Less, Ordering::Equal) => {
                self.intervals.insert(i + 1, Interval::new(start, interval.end, interval.bookings + 1));
                self.intervals[i].end = start;
                self.max = cmp::max(self.max, self.intervals[i].bookings + 1);
            }

            (_, Ordering::Equal, Ordering::Greater) => {
                self.intervals.insert(i + 1, Interval::new(end, interval.end, interval.bookings));
                self.intervals[i].end = end;
                self.intervals[i].bookings += 1;
                self.max = cmp::max(self.max, self.intervals[i].bookings);
            }


            (_, Ordering::Less, Ordering::Less) => {
                self.intervals.insert(i + 1, Interval::new(start, interval.end, interval.bookings));
                self.intervals[i].end = start;
                self.combine(i + 1, start, end);
            }

            (_, Ordering::Equal, Ordering::Less) => {
                self.intervals[i].bookings += 1;
                self.combine(i + 1, self.intervals[i].end, end);
                self.max = cmp::max(self.max, self.intervals[i].bookings);
            }

            _ => todo!()
        }
    }

    fn clear(&mut self) {
        self.intervals.clear();
        self.intervals.push(Interval::new(0, 10i32.pow(9), 0));
        self.max = 0;
    }
}

struct MyCalendar {
    points: Vec<(i32, i32)>
}

impl MyCalendar {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        let s = self.points.partition_point(|x| x.0 < start);
        if s < self.points.len() && self.points[s].0 == start {
            self.points[s].1 += 1;
        } else {
            self.points.insert(s, (start, 1));
        }

        let e = self.points.partition_point(|x| x.0 < end);
        if e < self.points.len() && self.points[e].0 == end {
            self.points[e].1 -= 1;
        } else {
            self.points.insert(e, (end, -1));
        }

        self.points.iter().map(|p| p.1).fold((0, 0), |(acc, max), x| {
            (acc + x, cmp::max(max, acc + x))
        }).1
    }

    fn clear(&mut self) {
        self.points.clear();
    }
}


fn main() {
    let inputs1 = [(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)];
    let inputs2 = [(24, 40), (43, 50), (27, 43), (5, 21), (30, 40), (14, 29), (3, 19), (3, 14), (25, 39), (6, 19)];

    let mut calendar = MyCalendar::new();
    for (start, end) in inputs1 {
        println!("{:?}", calendar.book(start, end));
    }

    calendar.clear();
    println!();

    for (start, end) in inputs2 {
        println!("{:?}", calendar.book(start, end));
    }
}
