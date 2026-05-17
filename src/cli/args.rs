use std::path::PathBuf;

use pareg::Pareg;

use crate::{cli::help, err::Result};

#[derive(Debug, Default)]
pub struct Args {
    pub sources: Vec<PathBuf>,
    pub expand: bool,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Args::default();
        res.parse_inner(&mut args)?;
        Ok(res)
    }

    fn parse_inner(&mut self, args: &mut Pareg) -> Result<()> {
        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => help(),
                "-e" | "--expand" => self.expand = true,
                "-s" | "--source" => self.sources.push(args.next_arg()?),
                a if a.starts_with('-') => {
                    return Err(args
                        .err_unknown_argument()
                        .hint("Use `-s` to specify this as source file.")
                        .into());
                }
                _ => self.sources.push(args.cur_arg()?),
            }
        }

        Ok(())
    }
}
