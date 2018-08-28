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
            Arg::with_name("TRAIN_LABELS")
                .help("Training filenames and labels")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("TEST_LABELS")
                .help("Test filenames and labels")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("TRAIN_CORPUS")
                .help("Zipped directory containing files for training (default: ./train.zip)")
                .index(3)
                .required(false),
        )
        .arg(
            Arg::with_name("TEST_CORPUS")
                .help("Zipped directory containing files for testing (default: ./test.zip)")
                .index(4)
                .required(false),
        )
        .get_matches()
}
