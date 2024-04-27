// Library
use chrono::{Duration, Local, NaiveTime};

// -------
// SEGMENT
// -------

/// A struct representing a segment of time
pub struct Segment {
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub duration: Duration,
}

impl Segment {
    /// Create a new segment with the specified start and end times
    fn new(start: NaiveTime, end: NaiveTime) -> Self {
        Self {
            start,
            end,
            duration: end.signed_duration_since(start),
        }
    }

    /// Get the time range of the segment in the format "HH:MM - HH:MM"
    pub fn time(&self) -> String {
        return format!("{} - {}", self.start, self.end);
    }
}

/// Get the segment number for the given time
pub fn get_current_segment(segments: &Vec<Segment>) -> usize {
    let time = Local::now().time();
    for (i, segment) in segments.iter().enumerate() {
        if time >= segment.start && time <= segment.end {
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
    start: NaiveTime,
    end: NaiveTime,
    duration: Duration,
}

impl Day {
    /// Create a new day with the default start and end times (00:00 - 23:59)
    pub fn new() -> Self {
        let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        Self {
            start,
            end,
            duration: end.signed_duration_since(start),
        }
    }

    /// Create a new day with the specified start and end times
    pub fn new_with(start: NaiveTime, end: NaiveTime) -> Self {
        Self {
            start,
            end,
            duration: end.signed_duration_since(start),
        }
    }

    /// Divide the day into segments of equal duration based on the given number of divisions
    pub fn divide(&self, divisions: u32) -> Vec<Segment> {
        let mut segments = Vec::new();

        // Calculate the duration of each segment
        let segment_duration = self.duration / divisions as i32;

        // Create segments based on the calculated duration
        let mut start = self.start;
        for _ in 0..divisions {
            let end = start + segment_duration;
            segments.push(Segment {
                start,
                end,
                duration: segment_duration,
            });
            start = end;
        }

        return segments;
    }
}
