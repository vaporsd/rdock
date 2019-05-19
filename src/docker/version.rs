

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Version {
	Version: String,
	ApiVersion: String,
	GitCommit: String,
	GoVersion: String,
	Os: String,
	Arch: String,
	KernelVersion: String,
	BuildTime: String,
	Experimental: bool
}
