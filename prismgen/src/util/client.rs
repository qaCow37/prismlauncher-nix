use reqwest::{
	Client as ReqClient,
	ClientBuilder as ReqClientBuilder,
};
use serde::de::DeserializeOwned;
use super::Result;

pub struct Client {
	client: ReqClient,
}
impl Client {
	pub fn new() -> Result<Self> {
		Ok(Self {
			client: ReqClientBuilder::new()
				.user_agent("qaCow37/prismlauncher-nix/prismgen")
				.build()?
		})
	}

	pub async fn get_index<T>(&self, uid: &str) -> Result<T>
	where
		T: DeserializeOwned
	{
		Ok(self.client
			.get(format!("https://meta.prismlauncher.org/v1/{uid}/index.json"))
			.send()
			.await?
			.json()
			.await?
		)
	}
}

//
// Change Plan, index all 7 index points async for
//  - net.minecraft
//  - net.fabricmc.intermediary
//  - net.fabricmc.fabric-loader
//  - org.quiltmc.quilt-loader
//  - net.minecraftforge
//  - net.neoforged
//  - com.mumfrey.liteloader
// once at beginning, cache results of it in unified struct
// then generate on request for all versions
//
