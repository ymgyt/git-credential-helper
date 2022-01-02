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

pub struct Operator {}

impl Operator {
    pub fn new() -> Self {
        Operator {}
    }

    pub fn get_credential<R, W>(&mut self, reader: R, writer: W) -> anyhow::Result<()>
    where
        R: io::Read,
        W: io::Write,
    {
        // read
        // protocol=xxx
        // host=yyy
        eprintln!("start get operation");

        // get credential from somewhere

        // write
        // username=xxx
        // password=yyyyy
        Ok(())
    }
}
