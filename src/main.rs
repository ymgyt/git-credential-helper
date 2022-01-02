use git_credential_helper::{cli, Operation, Operator, StdErrLogger};

use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let cli = cli::parse();

    let operation = match Operation::from_str(&cli.operation) {
        Ok(operation) => operation,
        Err(_undefined_operation) => {
            // if we receive undefined operation, ignore it.
            return Ok(());
        }
    };

    let mut operator = Operator::new().with_logger(StdErrLogger);
    let (mut stdin, mut stdout) = (std::io::stdin(), std::io::stdout());

    match operation {
        Operation::Get => operator.get_credential(&mut stdin, &mut stdout)?,
        Operation::Store => {
            // do nothing
        }
        Operation::Erase => {
            // do nothing
        }
    }

    Ok(())
}
