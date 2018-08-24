use clap::{App, AppSettings, Arg, ArgMatches};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

pub fn parse_args() -> ArgMatches<'static> {
    App::new("final-frontier")
        .settings(DEFAULT_CLAP_SETTINGS)
        .arg(
            Arg::with_name("stopwords")
                .short("s")
                .long("stopwords")
                .value_name("STOPWORDS")
                .help("Flag to remove stopwords from search (default: false)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("TRAIN")
                .help("Train data")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("TEST")
                .help("Test data")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("TRAINDIR")
                .help("Directory containing files for training (default: ./train)")
                .index(3)
                .required(false),
        )
        .arg(
            Arg::with_name("TESTDIR")
                .help("Directory containing files for testing (default: ./test)")
                .index(4)
                .required(false),
        )
        .get_matches()
}
