# Data:
- 5 genres (romance, sci-fi, horror, erotica and detective), each approx of 100 texts. 
- Sources: site archiveofourown (fanfiction) and texts from Project Gutenberg
- Split into test and train set, so far approx 20/80
- **Possible additions and improvements**: data augmentation? mismatch learning? (doubtful we'll have time for this sadly)

# TODO:
- ndarray SVD
- classifier (rusty-machine Logistic Regression probably easiest, but we have plenty of material for KNN). rusty machine Naive Bayes doesn't seem to work well for our case
- if nothing of this works, Ann will dramatically try to implement a simple neural network
- function(s) for reading zipped models and directories (our current project is almost 1GB large!)
- refactoring code / breaking up into smaller functions (stopwords could be a start)
- commenting everything through
- writing unit tests
- writing a one-page system description paper
