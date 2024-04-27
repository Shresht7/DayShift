// ---------
// CONSTANTS
// ---------

// -------
// SEGMENT
// -------

/// A struct representing a segment of time with a start, end time
/// and a duration in seconds
pub struct Segment {
    pub start: chrono::NaiveTime,
    pub end: chrono::NaiveTime,
    pub duration: chrono::Duration,
}

impl Segment {
    /// Create a new segment with the specified start and end times
    fn new(start: chrono::NaiveTime, end: chrono::NaiveTime) -> Self {
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
pub fn get_segment_number(segments: &Vec<Segment>, time: chrono::NaiveTime) -> usize {
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
    start: chrono::NaiveTime,
    end: chrono::NaiveTime,
    duration: chrono::Duration,
}

impl Day {
    /// Create a new day with the default start and end times (00:00 - 23:59)
    pub fn new() -> Self {
        let start = chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap();
        let end = chrono::NaiveTime::from_hms_opt(09, 59, 59).unwrap();
        Self {
            start,
            end,
            duration: end.signed_duration_since(start),
        }
    }

    /// Create a new day with the specified start and end times
    pub fn new_with(start: chrono::NaiveTime, end: chrono::NaiveTime) -> Self {
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
        for i in 0..divisions {
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
