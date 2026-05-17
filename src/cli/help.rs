use termal::{gradient, printacln};

pub fn help() {
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature = gradient("BonnyAD9", (250, 50, 170), (180, 50, 240));

    printacln!(
        "Welcome in {'g i}plam{'_} by {signature}{'_}
Version {v}

{'g}Usage:
  {'c}plam {'gr}[{'dy}flags{'gr}] {'gr}[sources]{'_}
    Interpret code in the given sources. Multiple sources are effectively
    concatenated.

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Show this help.

  {'y}-e  --expand{'_}
    Show the fully expanded version.

  {'y}-s  --source {'w}<file>{'_}
    Add new source. The source name may start with `-`.

 “ {'i}Be strong, all you people of the land,’ declares the Lord,
   ‘and work. For I am with you,’ declares the Lord Almighty. {'_}”
                                                   {'w bold}✝ Haggai 2:4{'_}
"
    )
}
