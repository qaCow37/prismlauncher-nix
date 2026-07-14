use reqwest::Client;
use tokio::task::JoinSet;
use std::collections::HashMap;
use crate::error::Result;

struct Index {
	// uid->index.json
	pkgs: HashMap<String, PkgIndex>,
}

impl Index {
	pub async fn new(client: &Client) -> Result<Self> {
		#[derive(serde::Deserialize)]
		struct Response {
			packages: Vec<Package>,
		}
		#[derive(serde::Deserialize)]
		struct Package {
			uid: String
		}

		let rp: Response = client
			.get("https://meta.prismlauncher.org/v1/index.json")
			.send()
			.await?
			.json()
			.await?;

		let mut tasks = JoinSet::new();
		for Package{uid} in rp.packages {
			let c = client.clone();
			tasks.spawn(async move {
				let i = PkgIndex::new(
					&c,
					&uid
				).await?;
				Result::<_>::Ok((uid, i))
			});
		}

		let mut pkgs = HashMap::new();
		while let Some(res) = tasks.join_next().await {
			let (uid, pkg) = res??;
			pkgs.insert(uid, pkg);
		}
		Ok(Self {pkgs})
	}

	pub fn mcversion_of(&self, version: &PkgVersion) -> Option<String> {
		for req in &version.requires {
			if req.uid == "net.minecraft" {
				if let Some(v) = &req.version {
					return Some(v.clone());
				}
			}
		}
		// run in two separate loops to prefer
		// top level "net.minecraft" requires
		for req in &version.requires {
			if let Some(pkg) = self.pkgs.get(&req.uid) {
				for vers in &pkg.versions {
					if let Some(v) = self.mcversion_of(vers) {
						return Some(v);
					}
				}
			}
		}
		None
	}
}

struct PkgIndex {
	// Vec<(version, requires)
	versions: Vec<PkgVersion>,
}
struct PkgVersion {
	version: String,
	requires: Vec<PkgRequire>,
}
struct PkgRequire {
	uid: String,
	version: Option<String>,
}

impl PkgIndex {
	pub async fn new(client: &Client, uid: &str) -> Result<Self> {
		#[derive(serde::Deserialize)]
		struct Response {
			versions: Vec<Version>,
		}
		#[derive(serde::Deserialize)]
		struct Version {
			version: String,
			#[serde(default)]
			requires: Vec<Require>,
		}
		#[derive(serde::Deserialize)]
		struct Require {
			uid: String,
			equals: Option<String>,
		}

		let rp: Response = client
			.get(format!("https://meta.prismlauncher.org/v1/{uid}/index.json"))
			.send()
			.await?
			.json()
			.await?;
		Ok(Self {
			versions: rp.versions.into_iter()
				.map(|v| PkgVersion {
					version: v.version,
					requires: v.requires.into_iter()
						.map(|r| PkgRequire {
							uid: r.uid,
							version: r.equals,
						})
						.collect(),
				})
				.collect(),
		})
	}
}

#[derive(Clone, Debug)]
pub struct PrismIndex {
	pkgs: HashMap<String, Vec<Package>>,
}
#[derive(Clone, Debug)]
pub struct Package {
	version: String,
	mc: Option<String>,
}

impl Package {
	pub fn get_version(&self) -> &str {
		self.version.as_str()
	}
	pub fn get_mc_version(&self) -> Option<&str> {
		self.mc.as_deref()
	}
}

impl PrismIndex {
	pub async fn new(client: &Client) -> Result<Self> {
		let index = Index::new(client).await?;

		let mut pkgs = HashMap::new();
		for (uid, pkg) in &index.pkgs {
			let mut versions = Vec::new();
			for vers in &pkg.versions {
				versions.push(Package {
					version: vers.version.clone(),
					mc: index.mcversion_of(vers),
				});
			}
			pkgs.insert(uid.clone(), versions);
		}

		Ok(Self {pkgs})
	}

	pub fn get_pkgs(&self, uid: &str)
		-> Option<impl Iterator<Item=&Package>>
	{
		self.pkgs
			.get(uid)
			.map(|v| v.iter())
	}
	pub fn get_pkgs_for_mc(&self, uid: &str, version: &str)
		-> Option<impl Iterator<Item=&Package>>
	{
		self.get_pkgs(uid)
			.map(|i| i.filter(|p| {
				p.mc.as_deref() == Some(version)
			}))
	}
	pub fn get_latest_pkg_for_mc(&self, uid: &str, version: &str)
		-> Option<&Package>
	{
		use lenient_semver::Version;

		if let Some(iter) = self.get_pkgs_for_mc(uid, version) {
			iter.map(|p| (
					p,
					Version::parse(
						&p.version
					).unwrap(),
				))
				.max_by(|(_,a),(_,b)| a.cmp(b))
				.map(|(a,_)| a)
		} else {None}
	}
}
