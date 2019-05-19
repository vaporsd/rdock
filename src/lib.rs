/*
 * rdock: Yet another Docker API library in Rust.
 * This project is licensed under the GNU GPLv3 License.
 */

#[macro_use]
extern crate failure;
extern crate http;
extern crate url;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod errors;
pub mod docker;

pub use docker::Docker;
