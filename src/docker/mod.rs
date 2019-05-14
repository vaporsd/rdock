/*
 * rdock: Yet another Docker API library in Rust.
 * This project is licensed under the GNU GPLv3 License.
 */

use http::{StatusCode, Method, Request, Response};
use std::os::unix::net::UnixStream;
use std::net::TcpStream;
use url::Url;
use failure::Error;

use ::errors::RdockError;

enum Transport {
	Unix {
		path: String,
		stream: UnixStream
	},
	Tcp {
		host: String,
		port: u16,
		stream: TcpStream
	}
}

pub struct Docker {
	transport: Transport,
}

impl Docker {
	pub fn new() -> Result<Docker, Error> {
		Docker::connect("unix://var/run/docker.sock")
	}

	pub fn connect<S>(url: S) -> Result<Docker, Error>
			where S: Into<String> {
		let url = Url::parse(&url.into())?;
		let transport = match url.scheme() {
			"unix" => {
				let path = url.path().to_string();

				Transport::Unix {
					path: path.to_string(),
					stream: UnixStream::connect(path)?
				}
			},
			"tcp" => {
				let host = url.host()
					.ok_or(Error::from(RdockError::HostMissing))?;
				let port = url.port().unwrap_or(2375);

				Transport::Tcp {
					host: host.to_string(),
					port: port,
					stream: TcpStream::connect(
						format!("{}:{}", host, port).as_str())?
				}
			},
			_ => return Err(Error::from(
					RdockError::UnknownScheme(url.scheme().to_string()))
			)
		};

		Ok(Docker {
			transport: transport,
		})
	}

	pub fn request(&mut self, req: http::Request<()>)
			-> http::Response<()> {

		Response::builder()
			.status(StatusCode::NOT_FOUND)
			.body(()).unwrap()
	}
}
