// Library
use std::fmt::{self, Display, Formatter};

// External Library
use chrono::{Duration, Local, NaiveDateTime, NaiveTime};

// ----------
// TIME FRAME
// ----------

/// A struct representing a time frame
#[derive(Debug)]
pub struct TimeFrame {
    start: NaiveDateTime,
    duration: Duration,
}

impl TimeFrame {
    /// Create a new time frame with the specified start-time and duration
    pub fn new(start: NaiveTime, duration: u32) -> Self {
        let today = Local::now().naive_local();
        let start = today.date().and_time(start);
        let duration = Duration::hours(duration as i64);
        Self { start, duration }
    }

    /// Divide the day into segments of equal duration based on the given number of divisions
    pub fn divide(&self, divisions: u32) -> Vec<TimeFrame> {
        let mut segments = Vec::new();

        // Calculate the duration of each segment
        let segment_duration = self.duration / divisions as i32;

        // Create segments based on the calculated duration
        let mut start = self.start;
        for _ in 0..divisions {
            let end = start + segment_duration;
            segments.push(TimeFrame {
                start,
                duration: segment_duration,
            });
            start = end;
        }

        return segments;
    }
}

impl TimeSegment for TimeFrame {
    fn start(&self) -> NaiveDateTime {
        self.start
    }
    fn end(&self) -> NaiveDateTime {
        self.start + self.duration
    }
}

// Implement the Display trait for TimeFrame
impl Display for TimeFrame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.start().time().format("%H:%M"),
            self.end().time().format("%H:%M")
        )
    }
}

// ------------
// TIME SEGMENT
// ------------

/// A trait for time segments that have a start and end time
pub trait TimeSegment {
    fn start(&self) -> NaiveDateTime;
    fn end(&self) -> NaiveDateTime;
}

/// Get the segment number for the given time
pub fn get_current_segment<T: TimeSegment>(segments: &[T]) -> usize {
    let time = Local::now().naive_local();
    for (i, segment) in segments.iter().enumerate() {
        if time >= segment.start() && time < segment.end() {
            return i;
        }
    }
    if time >= segments.last().unwrap().end() {
        return segments.len() - 1;
    } else if time < segments.first().unwrap().start() {
        return 0;
    } else {
        return 0;
    }
}
