# speakit
Practice Italian pronounciation: Search italian text and list words containing a substring such as "ce"

I wanted to practice sounds in reasonably common or not completely random italian words. 
Pronounciation is a key thing to get started with Italian.

To use it:

Git clone and from command line run `cargo run`

When prompted enter a substring e.g. "ce". Upto 15 words will be listed.

Example: 

`Type a substring or press q to quit:` \
`type ce press enter` 

Output: 

ce \
invece \
dice \
cuccetta \
voce \
certo \
cuccette \
francese \
luce \
francesi \
cento \
cerca \
faceva \
necessario \
semplice \
fece 

Alternatively, git clone repo and run `cargo build`. You will need to have a "data.txt" file containing your text in the same folder as the binary.

Then run `./speakit_rust` on command line. 

See below todo list.

Note: 

Replace data.txt with whatever text file you want to use and put it in the project root. 

I'm using Primo Levi "If this is a man" / "Se questo è un uomo" as a source to get frequently used words.

I had it to hand. But has a few random german words for obvious reasons. Which doesn't matter here too much.

TODO:

1. Make an executable with default better source file. Although this "works" for what I need.
2. Make an executable that takes a text source file name as an argument. Can add flags for number of words etc. 
3. Make a simple webpage to search and get results.


