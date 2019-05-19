extern crate rdock;
extern crate http;

use rdock::Docker;
use http::{Request, Method};

#[test]
fn version()
{
	let mut d = match Docker::new() {
		Ok(docker) => docker,
		Err(e) => panic!("{}", e)
	};

	panic!("{:#?}", d.rawrequest(Method::GET, "/version", None));
}
