/*
 * rdock: Yet another Docker API library in Rust.
 * This project is licensed under the GNU GPLv3 License.
 */

mod version;
pub use self::version::Version;

use serde::{Serialize, Deserialize};
use std::{os::unix::net::UnixStream, net::TcpStream};
use std::io::{Write, Read};
use http::Request;
use url::Url;

use failure::Error;
use ::errors::RdockError;

enum Transport {
	Unix(UnixStream),
	Tcp(TcpStream)
}

pub struct Docker {
	transport: Transport,
}

impl Docker {
	#[inline]
	pub fn new() -> Result<Docker, Error> {
		Docker::connect("unix://var/run/docker.sock")
	}

	pub fn connect<S>(url: S) -> Result<Docker, Error>
			where S: Into<String> {
		let url = Url::parse(&url.into())?;
		let transport = match url.scheme() {
			"unix" => {
				let path = url.path().to_string();

				Transport::Unix(UnixStream::connect(path)?)
			},
			"tcp" => {
				let host = url.host()
					.ok_or(Error::from(RdockError::HostMissing))?;
				let port = url.port().unwrap_or(2375);

				Transport::Tcp(TcpStream::connect(
						format!("{}:{}", host, port).as_str())?)
			},
			_ => return Err(Error::from(
					RdockError::UnknownScheme(url.scheme().to_string()))
			)
		};

		Ok(Docker {
			transport: transport,
		})
	}

	pub fn rawrequest(self, method: http::Method,
			uri: &str, body: Option<String>) -> Result<String, Error> {
		let headers = [
			"Content-Type: application/json",
			"Host: dummy-host"
		];
		let mut request = [
			format!("{} {} HTTP/1.1", method.as_str(), uri),
			format!("{}", headers.join("\r\n")),
			String::from("\r\n")
		].join("\r\n");
		if body.is_some() {
			request.push_str(body.unwrap().as_str());
		}
		request.push_str("\r\n");
		let mut response = String::new();

		match self.transport {
			Transport::Unix(mut stream) => {
				stream.write(request.as_bytes()).unwrap();
				stream.read_to_string(&mut response).unwrap();
			},
			Transport::Tcp(mut stream) => {
				stream.write(request.as_bytes()).unwrap();
				stream.read_to_string(&mut response).unwrap();
			},
		};
		Ok(response)
	}

	pub fn request<T>(self, method: http::Method, uri: &str) -> T
			where T: for <'de> serde::Deserialize<'de> {
//		let response = self.rawrequest(method, uri, None);
		let response = String::from("Lol");
		serde_json::from_str(&response).unwrap()
	}
}
