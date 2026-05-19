use std::path::PathBuf;

use pareg::Pareg;

use crate::{cli::help, err::Result};

/// Arguments pared by plam.
#[derive(Debug)]
pub struct Args {
    /// The source files.
    pub sources: Vec<PathBuf>,
    /// Maximum size of apply cache.
    pub cache_limit: usize,
}

impl Args {
    /// Parse arguments.
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Args::default();
        res.parse_inner(&mut args)?;
        Ok(res)
    }

    fn parse_inner(&mut self, args: &mut Pareg) -> Result<()> {
        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => help(),
                "-s" | "--source" => self.sources.push(args.next_arg()?),
                "-c" | "--cache-limit" => {
                    self.cache_limit = args.next_arg()?
                }
                "--unlimited-cache" => self.cache_limit = usize::MAX,
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

impl Default for Args {
    fn default() -> Self {
        Self {
            sources: Default::default(),
            cache_limit: 10000,
        }
    }
}
