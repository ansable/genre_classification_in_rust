// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.
// Note:        Makes heavy use of code provided by the course lecturer, Daniël de Kok, in one of previous assignments.

/// Module for parsing command line arguments.

use clap::{App, AppSettings, Arg, ArgMatches};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

pub fn parse_args() -> ArgMatches<'static> {
    App::new("final-frontier")
        .settings(DEFAULT_CLAP_SETTINGS)
        .arg(
            Arg::with_name("autopilot")
                .short("a")
                .long("autopilot")
                .value_name("AUTOPILOT")
                .help("Use pretrained models instead of going through training process")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("stopwords")
                .short("s")
                .long("stopwords")
                .value_name("STOPWORDS")
                .help("Remove stopwords from search (default: false)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("TRAIN_CORPUS")
                .help("Zipped directory containing files for training (default: models/train.zip)")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("TEST_CORPUS")
                .help("Zipped directory containing files for testing (default: models/test.zip)")
                .index(2)
                .required(false),
        )
        .arg(
            Arg::with_name("TRAIN_LABELS")
                .help("Training filenames and labels (default: labels_train.txt)")
                .index(3)
                .required(false),
        )
        .arg(
            Arg::with_name("TEST_LABELS")
                .help("Test filenames and labels (default: labels_test.txt)")
                .index(4)
                .required(false),
        )
        .get_matches()
}
