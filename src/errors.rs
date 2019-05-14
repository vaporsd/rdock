/*
 * rdock: Yet another Docker API library in Rust.
 * This project is licensed under the GNU GPLv3 License.
 */

#[derive(Debug, Fail)]
pub enum RdockError {
	#[fail(display = "The scheme '{}' is not supported", _0)]
	UnknownScheme(String),
	#[fail(display = "The host was missing in the url")]
	HostMissing,
}
