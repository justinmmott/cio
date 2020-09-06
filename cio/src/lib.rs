pub mod airtable;
pub mod applicants;
pub mod auth0;
pub mod code_that_should_be_generated;
pub mod configs;
pub mod core;
pub mod db;
pub mod journal_clubs;
pub mod mailing_list;
pub mod models;
pub mod rfds;
pub mod schema;
pub mod slack;
pub mod utils;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;
