//! git credential INPUT/OUTPUT FORMAT
//! https://git-scm.com/docs/git-credential#IOFMT

use nom::{bytes, character, combinator, multi, sequence, IResult};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GitCredentialAttribute {
    Protocol(String), // e.g https
    Host(String),
    Path(String),
    Username(String),
    Password(String),
    Url(String),
    Etc(String, String), // catch all
}

impl fmt::Display for GitCredentialAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitCredentialAttribute::Protocol(s) => write!(f, "protocol={}", s),
            GitCredentialAttribute::Host(s) => write!(f, "host={}", s),
            GitCredentialAttribute::Path(s) => write!(f, "path={}", s),
            GitCredentialAttribute::Username(s) => write!(f, "username={}", s),
            GitCredentialAttribute::Password(s) => write!(f, "password={}", s),
            GitCredentialAttribute::Url(s) => write!(f, "url={}", s),
            GitCredentialAttribute::Etc(key, value) => write!(f, "{}={}", key, value),
        }
    }
}

/// parse git credential helper get command input provided by git command.
pub fn parse_git_credential_attributes(input: &str) -> anyhow::Result<Vec<GitCredentialAttribute>> {
    git_credential_attributes(input)
        .map(|(_remain, attrs)| attrs)
        .map_err(|err| anyhow::anyhow!("{}", err))
}

/// The key may contain any bytes except =, newline, or NUL.  
fn key(input: &str) -> IResult<&str, &str> {
    bytes::complete::is_not("=\n")(input)
}

// The value may contain any bytes except newline or NUL.
fn value(input: &str) -> IResult<&str, &str> {
    bytes::complete::take_till1(|c: char| c == '\n')(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
    sequence::separated_pair(key, bytes::complete::tag("="), value)(input)
}

fn git_credential_attribute(input: &str) -> IResult<&str, GitCredentialAttribute> {
    combinator::map(key_value, |(key, value)| {
        let value = value.to_owned();
        match key {
            "protocol" => GitCredentialAttribute::Protocol(value),
            "host" => GitCredentialAttribute::Host(value),
            "path" => GitCredentialAttribute::Path(value),
            "username" => GitCredentialAttribute::Username(value),
            "password" => GitCredentialAttribute::Password(value),
            "url" => GitCredentialAttribute::Url(value),
            etc => GitCredentialAttribute::Etc(etc.to_owned(), value),
        }
    })(input)
}

fn git_credential_attributes(input: &str) -> IResult<&str, Vec<GitCredentialAttribute>> {
    sequence::terminated(
        multi::separated_list1(character::complete::line_ending, git_credential_attribute),
        character::complete::line_ending,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key() {
        assert_eq!(key("key=value"), Ok(("=value", "key")),);
    }

    #[test]
    fn parse_value() {
        assert_eq!(value("value\n"), Ok(("\n", "value")));
    }

    #[test]
    fn parse_key_value() {
        assert_eq!(key_value("key=value\nxxx"), Ok(("\nxxx", ("key", "value"))));
    }

    #[test]
    fn parse_git_credential_attribute() {
        assert_eq!(
            git_credential_attribute("protocol=https\nxxx"),
            Ok((
                "\nxxx",
                GitCredentialAttribute::Protocol("https".to_owned())
            ))
        );
        assert_eq!(
            git_credential_attribute("host=github.com\nxxx"),
            Ok((
                "\nxxx",
                GitCredentialAttribute::Host("github.com".to_owned())
            ))
        );
    }

    #[test]
    fn parse_git_credential_attributes() {
        let input = "protocol=https\nhost=github.com\n\n";

        assert_eq!(
            git_credential_attributes(input),
            Ok((
                "\n",
                vec![
                    GitCredentialAttribute::Protocol(String::from("https")),
                    GitCredentialAttribute::Host(String::from("github.com")),
                ]
            )),
        );
    }
}
