pub const PROGRAM_MESSAGE: &'static str = r###"$ run repo-matome

    #1. create $HOME/Downloads/repo-matome (if the directory exists, this phase skip)
    #2. create $HOME/Downloads/repo-matome/index.html (if the index.html exists, this phase skip)
    #3. scraping https://github.com and collect up to `100` repositories
    #4. save json file at $HOME/Downloads/repo-matome/XXXXX.json
    --- open index.html (if you are linux user, open index.html manually) --- 
    #5. select $HOME/Downloads/repo-matome/XXXXX.json
    #6. print json data.
"###;

pub const SELECT_LANGUAGES: &'static str = r###"$ language you want to search

    1)  C        11) JavaScript   21) Common Lisp       
    2)  C++      12) TypeScript   22) Perl
    3)  Rust     13) Ruby         23) SQL
    4)  Go       14) PHP          24) D
    5)  Swift    15) Dart         25) COBOL
    6)  Kotlin   16) Clojure      26) Object-c
    7)  Java     17) Elixir       27) HTML
    8)  C#       18) F#           28) CSS
    9)  Python   19) Nim          
    10) R        20) Shell          
"###;

pub const SELECT_BOUND: &'static str = r###"$ range queries

    - search the repository with a specified range of stars
    - lower and uppder bounds. stars:XXX..XXX

        n..n ────> upper
        |  
        └────────> lower

    when [lower: 1000] [upper: 2000]  =>  ok. search star range 1000..2000
    when [lower: 500]  [upper: *]     =>  ok. search star range 500..*

    when [lower: 1000] [upper: 1000]  =>  error.  1000..1000 
    when [lower: 1000] [upper: 100]   =>  error. 
        the lower bound of the range (1000 .. 100) is greater than the upper bound
"###;

pub const SELECT_TOPIC: &'static str = r###"$ repository topic

    - input topic you are interested in
    for example, 

        cli web framework terminal tool git snippet network 
        benchmark proxy server client http crypto ...
"###;

pub const SELECT_HEADLESS: &'static str = r###"$ headless mode

    Enter       : hide      chrome browser
    Press `n`   : visualize chrome browser
"###;