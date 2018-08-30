// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

/// Module containing classifier evaluation metrics, namely precision, recall and F1-score.

use std;
use std::collections::HashSet;
use std::iter::FromIterator;

// Precision: out of the times a label was predicted, {return values} of the time the system was in fact correct
fn precision(
    gold: &Vec<std::string::String>,
    pred: &Vec<std::string::String>,
    label: std::string::String,
) -> f64 {
    assert_eq!(
        gold.len(),
        pred.len(),
        "Label vectors must have the same length"
    );

    let mut total_predicted_for_label = 0;
    let mut true_positive_for_label = 0;

    if !pred.contains(&label) {
        return 0.0;
    }

    for i in 0..pred.len() {
        if pred[i] == label {
            total_predicted_for_label += 1;
            if gold[i] == label {
                true_positive_for_label += 1;
            }
        }
    }
    true_positive_for_label as f64 / total_predicted_for_label as f64
}

// Recall: out of the times a label should have been predicted, {return value} of the labels were correctly predicted
fn recall(
    gold: &Vec<std::string::String>,
    pred: &Vec<std::string::String>,
    label: std::string::String,
) -> f64 {
    let mut total_gold_for_label = 0;
    let mut true_positive_for_label = 0;

    if !gold.contains(&label) {
        return 0.0;
    }

    for i in 0..gold.len() {
        if gold[i] == label {
            total_gold_for_label += 1;
            if pred[i] == label {
                true_positive_for_label += 1;
            }
        }
    }
    true_positive_for_label as f64 / total_gold_for_label as f64
}

// F1: harmonic average of precision and recall
fn f1_score(precision: f64, recall: f64) -> f64 {
    if precision + recall == 0.0 {
        return 0.0;
    }

    2.0 * precision * recall / (precision + recall)
}

// Return macro-averaged precision, recall and F1 score for given predictions
pub fn macro_averaged_evaluation(
    gold: Vec<std::string::String>,
    pred: Vec<std::string::String>,
) -> (f64, f64, f64) {
    let mut macro_averaged_precision = 0.0;
    let mut macro_averaged_recall = 0.0;

    // the following line assumes every label appears at least once in the training data
    let labels_set: HashSet<std::string::String> = HashSet::from_iter(gold.iter().cloned());

    for label in labels_set.iter() {
        macro_averaged_precision += precision(&gold, &pred, label.to_string());
        macro_averaged_recall += recall(&gold, &pred, label.to_string());
    }

    macro_averaged_precision = macro_averaged_precision / labels_set.len() as f64;
    macro_averaged_recall = macro_averaged_recall / labels_set.len() as f64;
    let macro_averaged_f1_score = f1_score(macro_averaged_precision, macro_averaged_recall);

    (
        macro_averaged_precision,
        macro_averaged_recall,
        macro_averaged_f1_score,
    )
}
