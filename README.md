# mdwiki
janky but fast custom mediawiki parser for an upcoming wikipedia racing game

## what?
it converts an extracted wikipedia dump (.xml) into a folder with one markdown file per article.

still very much a work in progress. lacks macro expansion, multi-language support (currently pt only).

I also plan to implement multithreading and other optimizations.

## benchmarks

it churns through `ptwiki-20221120-articles.xml` (8.7gib), from [ptwiki-20221120-pages-articles.xml.bz2](https://dumps.wikimedia.org/ptwiki/20221120) in about 3min, running single-threaded on an i7-8700
