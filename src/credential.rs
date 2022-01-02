use anyhow::{anyhow, Context};

pub struct Credential {
    pub(crate) user: String,
    pub(crate) password: String,
}

pub trait Provide {
    fn provide_by_host(&self, host: &str) -> anyhow::Result<Credential>;
}

pub struct EnvProvider;

impl Provide for EnvProvider {
    fn provide_by_host(&self, host: &str) -> anyhow::Result<Credential> {
        // currently only github supported
        if host != "github.com" {
            return Err(anyhow!("currently only github.com supported"));
        }

        let github_user =
            std::env::var("GITHUB_USER").context("environment variable GITHUB_USER required")?;
        let github_token =
            std::env::var("GITHUB_TOKEN").context("environment variable GITHUB_TOKEN required")?;

        Ok(Credential {
            user: github_user,
            password: github_token,
        })
    }
}

#[cfg(test)]
pub(crate) struct DummyProvider {
    pub(crate) user: String,
    pub(crate) password: String,
}

#[cfg(test)]
impl Provide for DummyProvider {
    fn provide_by_host(&self, _host: &str) -> anyhow::Result<Credential> {
        Ok(Credential {
            user: self.user.clone(),
            password: self.password.clone(),
        })
    }
}
