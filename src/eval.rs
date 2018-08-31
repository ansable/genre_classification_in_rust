// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

/// Module containing classifier evaluation metrics, namely precision, recall and F1 score.
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
        // this is to avoid division by zero if some label was never predicted
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
    assert_eq!(
        gold.len(),
        pred.len(),
        "Label vectors must have the same length"
    );
    let mut total_gold_for_label = 0;
    let mut true_positive_for_label = 0;

    if !gold.contains(&label) {
        // this is to prevent division by zero if the label is not actually represented in the test set
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

// F1 score: harmonic average of precision and recall
fn f1_score(precision: f64, recall: f64) -> f64 {
    assert!(precision >= 0.0 && recall >= 0.0, "Precision and recall may not be negative");
    if precision + recall == 0.0 {
        return 0.0;
    }

    2.0 * precision * recall / (precision + recall)
}

// Returns macro-averaged precision, recall and F1 score for given predictions
pub fn macro_averaged_evaluation(
    gold: &Vec<std::string::String>,
    pred: &Vec<std::string::String>,
) -> (f64, f64, f64) {
    let mut macro_averaged_precision = 0.0;
    let mut macro_averaged_recall = 0.0;

    // the following line assumes every label appears at least once in the test data
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

#[cfg(test)]
mod tests {
    use super::{precision, recall, f1_score, macro_averaged_evaluation};

    #[test]
    fn precision_recall_test1(){
        let gold = vec![String::from("detective"), String::from("detective"), String::from("detective"), String::from("detective")];
        let pred = vec![String::from("detective"), String::from("erotica"), String::from("scifi"), String::from("horror")];
        assert_eq!(precision(&gold, &pred, String::from("detective")), 1.0);
        assert_eq!(recall(&gold, &pred, String::from("detective")), 0.25);
    }

    #[test]
    fn precision_recall_test2(){
        let gold = vec![String::from("erotica"), String::from("detective")];
        let pred = vec![String::from("erotica"), String::from("erotica")];
        assert_eq!(precision(&gold, &pred, String::from("erotica")), 0.5);
        assert_eq!(recall(&gold, &pred, String::from("erotica")), 1.0);
    }

    #[test]
    #[should_panic] // vectors should have the same length
    fn precision_recall_test3(){
        let gold = vec![String::from("erotica")];
        let pred = vec![String::from("erotica"), String::from("erotica")];
        assert_eq!(precision(&gold, &pred, String::from("erotica")), 0.5);
        assert_eq!(recall(&gold, &pred, String::from("erotica")), 1.0);
    }

    #[test]
    fn f1_score_test1(){
        let precision = 0.2;
        let recall = 0.6;
        assert_eq!(f1_score(precision, recall), 0.3);
    }

    #[test]
    #[should_panic] // neither precision nor recall may be negative
    fn f1_score_test2(){
        let precision = 0.2;
        let recall = -0.6;
        assert_eq!(f1_score(precision, recall), 0.3);
    }

    #[test]
    fn macro_averaged_evaluation_test(){
        let gold = vec![String::from("detective"), String::from("detective"), String::from("horror")];
        let pred = vec![String::from("detective"), String::from("horror"), String::from("horror")];
        assert_eq!(macro_averaged_evaluation(&gold, &pred), (0.75, 0.75, 0.75));
    }
}
