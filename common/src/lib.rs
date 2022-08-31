mod regex;

use std::{collections::HashMap, ops::Range};

pub use regex::Regex;

#[derive(Debug)]
pub struct Match<'t, I> {
    input: &'t [I],
    start: usize,
    end: usize,
}

impl<'t, I> Match<'t, I> {
    /// Returns the starting offset of the match in the haystack.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending offset of the match in the haystack.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the range over the starting and ending offsets of the match in the haystack.
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the matched sub-slice.
    pub fn values(&self) -> &'t [I] {
        &self.input[self.range()]
    }
}

#[derive(Debug)]
struct CaptureLocation {
    pub start: usize,
    pub end: usize,
}

/// Captures represents a group of captured sub-slice for a single match.
///
/// The 0th capture always corresponds to the entire match. Each subsequent index corresponds to the next capture group in the regex.
///  If a capture group is named, then the matched sub-slice is also available via the name method.
///  (Note that the 0th capture is always unnamed and so must be accessed with the get method.)
///
/// `'t` is the lifetime of the matched slice.
#[derive(Debug)]
pub struct Captures<'t, I> {
    input: &'t [I],
    capture_locations: Vec<CaptureLocation>,
    named_capture_index: HashMap<String, usize>,
}

impl<'t, I> Captures<'t, I> {
    /// Returns the match associated with the capture group at index i.
    /// If i does not correspond to a capture group, or if the capture group did not participate in the match, then None is returned.
    pub fn get(&self, index: usize) -> Option<Match<'t, I>> {
        self.capture_locations.get(index).map(|location| Match {
            input: self.input,
            start: location.start,
            end: location.end,
        })
    }

    /// Returns the match for the capture group named name. If name isn’t a valid capture group or didn’t match anything, then None is returned.
    pub fn name(&self, name: &str) -> Option<Match<'t, I>> {
        self.named_capture_index
            .get(name)
            .and_then(|idx| self.get(*idx))
    }

    /// Returns the total number of capture groups (even if they didn’t match).
    /// This is always at least 1, since every regex has at least one capture group that corresponds to the full match.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.capture_locations.len()
    }
}

pub trait CompiledRegex<I> {
    /// Returns true if and only if match the entire input slice.
    fn is_full_match(&self, input: &[I]) -> bool;

    /// Returns the capture groups corresponding to the leftmost-first match in text.
    /// Capture group 0 always corresponds to the entire match. If no match is found, then None is returned.
    fn captures<'t>(&self, input: &'t [I]) -> Option<Captures<'t, I>>;
}
