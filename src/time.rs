// Library
use std::fmt::{self, Display, Formatter};

// External Library
use chrono::{Duration, Local, NaiveDateTime, NaiveTime};

// -------
// SEGMENT
// -------

/// A struct representing a segment of time
pub struct Segment {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

// Implement the Display trait for the Segment struct
impl Display for Segment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.start.time().format("%H:%M"),
            self.end.time().format("%H:%M")
        )
    }
}

/// Get the segment number for the given time
pub fn get_current_segment(segments: &Vec<Segment>) -> usize {
    let time = Local::now().naive_local();
    for (i, segment) in segments.iter().enumerate() {
        if time >= segment.start && time < segment.end {
            return i;
        }
    }
    return 0;
}

// ---
// DAY
// ---

/// A struct representing a day with a start and end time
pub struct Day {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Day {
    /// Create a new day with the specified start and end times
    pub fn new_with(start: NaiveTime, end: NaiveTime) -> Self {
        let date = Local::now().naive_local().date();

        let end = if end <= start {
            NaiveDateTime::new(date, end) + Duration::days(1)
        } else {
            NaiveDateTime::new(date, end)
        };
        let start = NaiveDateTime::new(date, start);

        Self { start, end }
    }

    /// Get the duration of the day
    fn duration(&self) -> Duration {
        return self.end.signed_duration_since(self.start);
    }

    /// Divide the day into segments of equal duration based on the given number of divisions
    pub fn divide(&self, divisions: u32) -> Vec<Segment> {
        let mut segments = Vec::new();

        // Calculate the duration of each segment
        let segment_duration = self.duration() / divisions as i32;

        // Create segments based on the calculated duration
        let mut start = self.start;
        for _ in 0..divisions {
            let end = start + segment_duration;
            segments.push(Segment { start, end });
            start = end;
        }

        return segments;
    }
}
