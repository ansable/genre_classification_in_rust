# Data:
- 5 genres (romance, sci-fi, horror, erotica and detective), each approx of 100 texts. 
- Sources: site archiveofourown (fanfiction) and texts from Project Gutenberg
- Splitted to test and train set, so far approx 20/80
- **Possible additions and improvements**: data augmentation? mismatch learning?

# TODO:
- ndarray SVD
- parellelism: from command line or from within program?
- how to save our model/intermediate results: https://serde.rs/#data-formats (pickle should work well for what we need)
- think about some sort of normalisation (lowercasing, lemmatisation? Would slow things down but reduce data sparsity)
- KNN (Daniel's/ours implementation)
- if results of KNN aren't good: Logistic Regression / Naive Bayes
- rusty machine has a very straightforward interface, it would be easy to try it out quickly
(https://athemathmo.github.io/rusty-machine/doc/rusty_machine/learning/naive_bayes/index.html)
- if nothing of this works, Ann will dramatically try to implement a simple neural network
