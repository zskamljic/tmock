//! # mock
//!
//! Mock is a crate used to mock torrent uploads to given announcers.
mod announcer;
mod client;
mod compact_trackers;
mod id_generator;
mod key_generator;
#[cfg(test)]
mod tests;
mod tracker_updates;

pub use announcer::Announcer;
pub use client::Client;
