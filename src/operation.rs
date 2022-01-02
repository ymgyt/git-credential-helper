use crate::{
    credential,
    log::{Log, NopLogger},
    parse,
};

use std::{io, str::FromStr};

// if we receive an operation that is not defined
// we must ignore it, so we don't use the clap arg enum.
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Get,
    Store,
    Erase,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Operation::Get),
            "store" => Ok(Operation::Store),
            "erase" => Ok(Operation::Erase),
            etc => Err(etc.to_owned()),
        }
    }
}

pub struct Operator<L, P> {
    logger: L,
    provider: P,
}

impl Operator<NopLogger, credential::EnvProvider> {
    pub fn new() -> Operator<NopLogger, credential::EnvProvider> {
        Operator {
            logger: NopLogger,
            provider: credential::EnvProvider,
        }
    }
}

impl Default for Operator<NopLogger, credential::EnvProvider> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L, P> Operator<L, P>
where
    L: Log,
    P: credential::Provide,
{
    pub fn with_logger<L2: Log>(self, logger: L2) -> Operator<L2, P> {
        Operator {
            logger,
            provider: self.provider,
        }
    }

    pub fn with_provider<P2: credential::Provide>(self, provider: P2) -> Operator<L, P2> {
        Operator {
            logger: self.logger,
            provider,
        }
    }

    pub fn get_credential<R, W>(&mut self, mut reader: R, mut writer: W) -> anyhow::Result<()>
    where
        R: io::Read,
        W: io::Write,
    {
        self.logger.log("credential helper invoked by git");

        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let attributes = match parse::parse_git_credential_attributes(&buf) {
            Ok(attributes) => attributes,
            Err(err) => return Err(anyhow::anyhow!("err: {:?} input: {}", err, buf)),
        };

        self.logger.log(&format!("parse: {:?}", attributes));

        let host = attributes
            .iter()
            .find_map(|attr| {
                if let parse::GitCredentialAttribute::Host(ref host) = attr {
                    Some(host.to_owned())
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("host not found in input: {}", buf))?;

        let credential = self.provider.provide_by_host(&host)?;

        writeln!(
            &mut writer,
            "{}",
            parse::GitCredentialAttribute::Username(credential.user)
        )?;
        writeln!(
            &mut writer,
            "{}",
            parse::GitCredentialAttribute::Password(credential.password)
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_credential() {
        let dummy_privder = credential::DummyProvider {
            user: String::from("xxx"),
            password: String::from("yyy"),
        };
        let mut ops = Operator::new().with_provider(dummy_privder);

        let input = "protocol=https\nhost=github.com\n\n";
        let expect = "username=xxx\npassword=yyy\n";
        let mut buff = Vec::new();

        ops.get_credential(input.as_bytes(), &mut buff).unwrap();

        assert_eq!(String::from_utf8_lossy(buff.as_slice()).as_ref(), expect);
    }
}
