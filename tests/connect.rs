extern crate rdock;

use rdock::Docker;

#[test]
fn default()
{
	match Docker::new() {
		Ok(_) => {},
		Err(e) => panic!("{}", e)
	};
}

#[test]
fn unixconnect()
{
	match Docker::connect("unix://var/run/docker.sock") {
		Ok(_) => {},
		Err(e) => panic!("{}", e)
	};
}

#[test]
fn tcpconnect()
{
	match Docker::connect("tcp://127.0.0.1:2375") {
		Ok(_) => {},
		Err(e) => panic!("{}", e)
	};
}

#[test]
fn tcpconnect_noport()
{
	match Docker::connect("tcp://127.0.0.1") {
		Ok(_) => {},
		Err(e) => panic!("{}", e)
	};
}
