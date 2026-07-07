use super::client::Client;
use super::Result;
use serde::Deserialize;
use lenient_semver::Version as SemVer;

#[derive(Deserialize)]
pub struct Response {
	versions: Vec<Version>
}
#[derive(Deserialize)]
struct Version {
	// recommended: bool,
	version: String,
	requires: Vec<Require>,
}
#[derive(Deserialize)]
struct Require {
	uid: String,
	// suggest: Option<String>,
	equals : Option<String>,
}

impl Response {
	pub async fn new(client: &Client, uid: &str) -> Result<Self> {
		Ok(Self::from(client.get_index(
			uid
		).await?))
	}

	pub fn get_all_versions<'a>(&'a self) -> impl Iterator<Item = &'a str> {
		self.versions.iter()
			.map(|v| {
				v.version.as_str()
			})
	}
	pub fn get_latest_version<'a>(&'a self) -> Option<&'a str> {
		self.versions.iter()
			.map(|v| (
				&v.version,
				SemVer::parse(&v.version),
			))
			.max_by(|(_,a),(_,b)| a.cmp(b))
			.map(|(v,_)| v.as_str())
	}

	pub fn get_version_for_require<'a>(&'a self, uid: &str, vers: &str) -> Option<&'a str> {
		self.versions.iter()
			.filter(|v| v.requires.iter()
				.any(|r| {
					r.uid == uid
					&& r.equals.as_deref() == Some(vers)
				})
			).map(|v| (
				&v.version,
				SemVer::parse(&v.version),
			))
			.max_by(|(_,a),(_,b)| a.cmp(b))
			.map(|(v,_)| v.as_str())
	}
}
