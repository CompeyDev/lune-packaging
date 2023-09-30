use std::{collections::HashMap, str::FromStr, time::Duration};

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize};
use ureq::Response;
use url::Url;

use crate::types::LuneReleaseData;

#[derive(Debug, Default, Clone)]
pub struct GitHub {
	fetcher: Fetcher,
	repository: Repository,
}

impl GitHub {
	pub fn new(repo: (&str, &str)) -> Self {
		let fetcher = Fetcher::new();

		Self {
			fetcher: fetcher.to_owned(),
			repository: Repository::new(repo.0, repo.1, fetcher),
		}
	}

	pub fn fetch_releases(&self) -> Result<LuneReleaseData> {
		let api_uri = Url::from_str(self.repository.api_url().as_str())?.join("releases/")?.join("latest")?;

		Ok(match self.fetcher.fetch::<_, LuneReleaseData>(Method::Get, api_uri, true)? {
			FetchResult::Result(res) => res,
			FetchResult::Response(_) => {
				unreachable!("Fetcher returned Response instead of Result")
			}
		})
	}
}

#[derive(Debug, Default, Clone)]
pub struct Repository {
	fetcher: Fetcher,
	scope: String,
	repo: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RepositoryMeta {
	pub full_name: String,
	pub name: String,
	pub description: String,
	pub license: HashMap<String, String>,
	pub topics: Vec<String>,
	pub forks: u32,
	pub open_issues: u32,
	pub stars: u32,
}

impl Repository {
	pub fn new<T>(scope: T, repo: T, fetcher: Fetcher) -> Self
	where
		T: ToString,
	{
		Self {
			fetcher,
			scope: scope.to_string(),
			repo: repo.to_string(),
		}
	}

	pub fn scope(&self) -> &String {
		&self.scope
	}

	pub fn repo(&self) -> &String {
		&self.repo
	}

	pub fn api_url(&self) -> String {
		format!("https://api.github.com/repos/{}/{}/", self.scope, self.repo)
	}

	pub fn fetch_meta(&self) -> Result<RepositoryMeta> {
		Ok(
			match self
				.fetcher
				.fetch::<_, RepositoryMeta>(Method::Get, Url::from_str(self.api_url().as_str())?, true)?
			{
				FetchResult::Result(deserialized) => deserialized,
				FetchResult::Response(_) => {
					unreachable!("Fetcher returned Response instead of Result")
				}
			},
		)
	}
}

#[derive(Debug, Clone)]
pub struct Fetcher {
	client: ureq::Agent,
}

pub enum Method {
	Get,
	Post,
}

pub enum FetchResult<D> {
	Result(D),
	Response(Box<Response>),
}

impl Default for Fetcher {
	fn default() -> Self {
		Self::new()
	}
}

impl Fetcher {
	pub fn new() -> Self {
		Self {
			client: ureq::builder()
				.redirects(2)
				.https_only(true)
				.timeout(Duration::from_secs(30))
				.user_agent("lune-action/0.1.0")
				.build(),
		}
	}

	pub fn fetch<U, D>(&self, method: Method, uri: U, to_deserialize: bool) -> Result<FetchResult<D>>
	where
		U: Into<url::Url>,
		D: DeserializeOwned,
	{
		let method = match method {
			Method::Get => "GET",
			Method::Post => "POST",
		};

		let req = self.client.request_url(method, &uri.into());

		Ok(match to_deserialize {
			true => FetchResult::Result(serde_json::from_reader::<_, D>(req.call()?.into_reader())?),
			false => FetchResult::Response(Box::new(req.call()?)),
		})
	}
}
