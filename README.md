# Stage 1 (Anna - in process)

  - Collecting Corpora. (6 genres, 100 texts each, except romantic literature: but amount of texts aligned with other genres)
  - Separate to train and test (do we need dev?)
  - **Status**: Horror and romance are to be done
  - **Problems**: too old texts. **Propossed solution**: using fanfiction in specific genres to make text less old.
  - **Possible addition**: mismatch learning?


# Stage 2. Preprocessing (I propose to do a separate file or class with it)

  - *Main function* (**half-done: works for html**): combining all helper functions. You input there: true/false remove stopwords; it uses pattern match: go to three options: txt, html, other. 
  - 1. Txt -> move to txt_preprocessing function. Afterwards -> to tokenizer (here we have two options, about it later). 
  - 2. Html -> html-preprocessing; tokenizer;
  - 3. other -> error message.
  - *type_of_document* (**works**) - its very basic, so we could do without it. I like it aesthetically though.
  - *html-preprocessing* (**works**)
  - *txt-preprocessing* (**have to decide on our strategy. Proposing grep? in work**)
  - *stopwords* (**works**) - but i just realized that i apply it on strings and we will apply it on tokenized text, should be easy fix
  - *tokenize_text* (**works**)
  - *make_ngrams* (**works**) - probs dont need it, also i fucked up with output should fix it!

# Stage 2.5. Command line
(using: clap)
We gonna use command line, cause thanks to Peter now we know that apparently there are shit ton of pluses in doing that. *clap-clap*

# Stage 3. Algorithms (main part)
(using: scanlex for word tokenizer)
-Trying to work with (uni – n)grams; here use both versions with StopWords and NoStopWords
For big texts: splitting big novels in equal parts and assuming the same label for all of them.

Algorithms: (using: rust-tfidf)
- Using tf/idf (can be later united with KNN) (can be applied of both types of our corpora: it would be interesting if one of “stop words” would actually appear to be significant)

(using for KNN: our implementation or rusty library?)
- main problem with KNN – too huge distances; so we gonna take care of that with using singular value decomposition => get a dense matrix which has fewer dimensions ergo smaller distances
(using: ndarray)
- ndarray SVD
 
Also we gonna use somewhere: ordered float
