// ---------
// CONSTANTS
// ---------

/// The number of minutes in a day
pub const MINUTES_IN_DAY: u32 = 24 * 60;

// -------
// SEGMENT
// -------

/// A struct representing a segment of time with a start, end time
/// and a duration in minutes
pub struct Segment {
    pub start: u32,
    pub end: u32,
    pub duration: u32,
}

impl Segment {
    /// Create a new segment with the specified start and end times
    fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            duration: end - start,
        }
    }

    /// Get the time range of the segment in the format "HH:MM - HH:MM"
    pub fn time(&self) -> String {
        let start = format!("{:02}:{:02}", self.start / 60, self.start % 60);
        let end = format!("{:02}:{:02}", self.end / 60, self.end % 60);
        return format!("{} - {}", start, end);
    }
}

/// Get the segment number for the given time
pub fn get_segment_number(segments: &Vec<Segment>, time: u32) -> usize {
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
    start: u32,
    end: u32,
    duration: u32,
}

impl Day {
    /// Create a new day with the default start and end times (00:00 - 23:59)
    pub fn new() -> Self {
        Self {
            start: 0,
            end: MINUTES_IN_DAY,
            duration: MINUTES_IN_DAY,
        }
    }

    /// Create a new day with the specified start and end times
    pub fn new_with(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            duration: end - start,
        }
    }

    /// Divide the day into segments of equal duration based on the given number of divisions
    pub fn divide(&self, divisions: u32) -> Vec<Segment> {
        let mut segments = Vec::new();

        // Calculate the duration of each segment
        let segment_duration = (self.duration as f64 / divisions as f64).ceil() as u32;

        // Create segments based on the calculated duration
        for i in 0..divisions {
            let start = self.start + i * segment_duration;
            let end = start + segment_duration;
            segments.push(Segment {
                start,
                end,
                duration: segment_duration,
            });
        }

        return segments;
    }
}
