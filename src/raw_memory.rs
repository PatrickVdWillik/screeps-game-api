//! Interface for Screeps [`RawMemory`] global object.
//!
//! [`RawMemory`]: https://docs.screeps.com/api/#RawMemory

use serde::Deserialize;

use crate::macros::*;

#[derive(Deserialize, Debug)]
pub struct ForeignSegment {
    username: String,
    id: String,
    data: String,
}

js_deserializable!(ForeignSegment);

get_from_js!(get_active_segments -> {
    Object.keys(RawMemory.segments).map(Number)
} -> Vec<u32>);

/// Sets active segments (max 10 ids).
pub fn set_active_segments(ids: &[u32]) {
    assert!(
        ids.len() <= 10,
        "can't set more than 10 active segments at a time"
    );
    js! { @(no_return)
        RawMemory.setActiveSegments(@{ids});
    }
}

get_from_js!(get_segment(id: u32) -> {
    RawMemory.segments[@{id}]
} -> Option<String>);

pub fn set_segment(id: u32, data: &str) {
    js! { @(no_return)
        RawMemory.segments[@{id}] = @{data};
    }
}

/// This drops the reference to a segment; it doesn't affect the content of the
/// segment.
///
/// This is the equivalent of doing `delete RawMemory.segments[id]`. Again, this
/// only deletes the local view of the segment, not the serialized one. It may
/// be used to `set_segment` a new segment that wasn't part of the original 10
/// active segments.
pub fn drop_segment(id: u32) {
    js! { @(no_return)
        delete RawMemory.segments[@{id}];
    }
}

get_from_js!(get_foreign_segment -> {
    RawMemory.foreignSegment
} -> ForeignSegment);

/// Implements `RawMemory.setActiveForeignSegment`
///
/// To use the default public segment of `username` (as set with
/// [`set_default_public_segment`]), Use `None` instead of `Some(id)`.
///
/// To clear the foreign segment, pass the empty string `""` as a username.
pub fn set_active_foreign_segment(username: &str, id: Option<u32>) {
    if username == "" {
        js! { @(no_return)
            RawMemory.setActiveForeignSegment(null);
        }
    } else {
        match id {
            Some(id) => js! { @(no_return)
                RawMemory.setActiveForeignSegment(@{username}, @{id});
            },
            None => js! { @(no_return)
                RawMemory.setActiveForeignSegment(@{username});
            },
        };
    };
}

pub fn set_default_public_segment(id: u32) {
    js! { @(no_return)
        RawMemory.setDefaultPublicSegment(@{id});
    }
}

pub fn set_public_segments(ids: &[u32]) {
    js! { @(no_return)
        RawMemory.setPublicSegments(@{ids});
    }
}

get_from_js!(get -> {RawMemory.get()} -> String);

pub fn set(value: &str) {
    js! { @(no_return)
        RawMemory.set(@{value});
    }
}
