# repo-matome
command line and scraping tool to collect github repo efficiently

# Prerequisites
chrome web browser

# How to install and run

1. `git clone https://github.com/hibi221b/repo-matome.git`
2. `cd repo-matome`
3. `cargo run`

# Usage

```console
repo-matome 0.1.0
hibi221b
repo-matome efficiently collects github repositories, saves the results in json file, and displays them in index.html
under the Donwload directory.

USAGE:
    repo-matome

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

```

# How to use `repo-matome`

## First

- you select number(1~28) of languages that you want to search

```console
info: browser path is PRINT_YOUR_CHROME_PATH

$ run repo-matome

    #1. create $HOME/Downloads/repo-matome (if the directory exists, this phase skip)
    #2. create $HOME/Downloads/repo-matome/index.html (if the index.html exists, this phase skip)
    #3. scraping https://github.com and collect up to `100` repositories
    #4. save json file at $HOME/Downloads/repo-matome/XXXXX.json
    --- open index.html (if you are linux user, open index.html manually) --- 
    #5. select $HOME/Downloads/repo-matome/XXXXX.json
    #6. print json data.

$ language you want to search

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

▼ select number 1 ~ 28
3 <------------------------- select Rustlang
```

## Second

- you select lower bound of github repository stars(1000, 2500, 100000 etc..)

```console
$ range queries

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

▼ input lower bound
10000 <------------------------- stars:10000..XXX (star range)
```

## Third

- you select upper bound of github repository stars(1000, 2500, 100000 or *)
- set the upper bound to be greater than the lower bound

```console
▼ input upper bound
* <------------------------- stars:10000..*  (star range)
```

## Fourth

- you select topic of github repository
- You can select one or more keywords

```console
$ repository topic

    - input topic you are interested in
    for example, 

        cli web framework terminal tool git snippet network 
        benchmark proxy server client http crypto ...

▼ input topic words. If you don't need, press enter.
cli command-line tool  <------------------------------------ input keywords you want to search
```

## Fifth

- select headless mode

```console
$ headless mode

    Enter       : hide      chrome browser
    Press `n`   : visualize chrome browser

▼ Press enter: hide chrome, input `n`: visualize chrome
  <---------------- if you press enter, hide chrome browser
```

# Result

- save Rust repositories with more than 10000 stars in json files. (The language and number of stars can be changed. The maximum number of repositories that can be retrieved is 100.)
- create a repo-matome directory under $HOME/Downloads.
- create a new index.html there.
- open $HOME/Downloads/repo-matome/index.html local file.
- select the json file that contains the information obtained by scraping.
- display the contents on the browser.

```console
headless_mode: true
search query:  language:Rust stars:10000..*
============================================================================

info: setup successfully finished

info: 12 repository results
----------------------------------------------------------------------------

info: hit! 10 repositories
info: 1 page now
get:  https://github.com/996icu/996.ICU
get:  https://github.com/rust-lang/rust
get:  https://github.com/alacritty/alacritty
get:  https://github.com/BurntSushi/ripgrep
get:  https://github.com/sharkdp/bat
get:  https://github.com/xi-editor/xi-editor
get:  https://github.com/servo/servo
get:  https://github.com/rust-unofficial/awesome-rust
get:  https://github.com/libra/libra
get:  https://github.com/sharkdp/fd
info: go to the next page

info: hit! 2 repositories
info: 2 page now
get:  https://github.com/firecracker-microvm/firecracker
get:  https://github.com/yewstack/yew

info: 12 repositories acquired. ($HOME/Downloads/repo-matome/xxxxx.json)
----------------------------------------------------------------------------

info: scraping successfully finished.
info: open /Users/hibi221b/Downloads/repo-matome/index.html

```
