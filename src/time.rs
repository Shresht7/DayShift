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
    /// Get the start time of the segment
    fn start(&self) -> NaiveDateTime;
    /// Get the end time of the segment
    fn end(&self) -> NaiveDateTime;
}

pub trait CurrentSegment<T: TimeSegment> {
    /// Get the current segment based on the current time
    fn current(&self) -> &T;
    /// Get the index of the current segment based on the current time
    fn current_index(&self) -> usize;
}

impl<T: TimeSegment> CurrentSegment<T> for Vec<T> {
    fn current_index(&self) -> usize {
        let time = Local::now().naive_local();
        for (i, segment) in self.iter().enumerate() {
            if time >= segment.start() && time < segment.end() {
                return i;
            }
        }
        if time >= self.last().unwrap().end() {
            return self.len() - 1;
        } else {
            return 0;
        }
    }

    fn current(&self) -> &T {
        &self[self.current_index()]
    }
}
