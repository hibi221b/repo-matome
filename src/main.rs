#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate clap;

mod parse_arguments;
mod selector;
mod program_message;
mod app;
mod scraping;
mod index_html;
mod path_name;
mod select_languages_from_list;
mod select_bound;
mod select_topic;
mod select_headless_mode;

use parse_arguments::parse_arg;
use program_message::{PROGRAM_MESSAGE, SELECT_LANGUAGES, SELECT_BOUND, SELECT_TOPIC, SELECT_HEADLESS};
use app::App;
use scraping::scraping_github;
use index_html::REPO_MATOME_INDEX_HTML;
use path_name::{INDEX_HTML_PATH, REPO_MATOME_DIR_PATH};
use select_languages_from_list::{select_languages, LANG};
use select_bound::{select_lower_bound, select_uppder_bound};
use select_topic::select_topic;
use select_headless_mode::select_headless_mode;

use std::fs::{self, File};
use std::io;
use std::io::{Write};
use std::path::{Path};
use std::process;
use std::thread;
use std::time::Duration;

use console::style;
use chrono::Local;
use headless_chrome::browser;

fn setup_environment() -> io::Result<()> {
    if !REPO_MATOME_DIR_PATH.exists() {
        fs::create_dir(REPO_MATOME_DIR_PATH.as_path())?;
    }

    if !INDEX_HTML_PATH.exists() {
        let mut index_html = File::create(INDEX_HTML_PATH.as_path())?;
        index_html.write_all(REPO_MATOME_INDEX_HTML.as_bytes())?;
    }

    Ok(())
}

fn check_json_files_in_the_repo_matome_directory() -> u64 {
    let mut json_file_count = 0;
    for dir_entry in REPO_MATOME_DIR_PATH.read_dir().expect("cannt read dir") {
        if let Ok(entry) = dir_entry {
            let entry = entry.path().as_path().to_str().unwrap().to_string();
            if entry.ends_with(".json") && entry.contains("No-") && entry.contains("-lang_") && entry.contains("-range_") && entry.contains("-topics_") {
                json_file_count += 1;
            }
        }
    }

    match json_file_count {
        0 => 1, //for the first time `No-1` filename
        _ => json_file_count + 1 //after this, No-2, No-3, No-4...
    }
}

fn create_json_file(app: &App, file_number: u64) -> io::Result<()> {
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let topic = match app.topic.as_str() {
        "" => "None",
        _ => app.topic.as_str()
    };
    let topic = topic.to_string().replace(" ", "-");
    let json_path = format!("No-{No}_{Lang}-lang_{Low}-{Up}-range_{Topic}-topics_{Time}.json",
            No    = file_number,
            Lang  = app.search_lang,
            Low   = app.lower_bound,
            Up    = app.upper_bound,
            Topic = topic,
            Time  = now
    );

    let mut abs_json_path = REPO_MATOME_DIR_PATH.clone();
    abs_json_path.push(json_path);

    let serialize = serde_json::to_vec_pretty(&app.contents)?;
    let mut file = File::create(&abs_json_path)?;
    file.write_all(&serialize)?;

    Ok(())
}

fn spawn_index_html(path: &Path) {
    if cfg!(target_os = "windows"){
        process::Command::new("start")
            .arg(path)
            .spawn()
            .expect(&format!("cannot open {}", path.to_str().unwrap()));

    } else if cfg!(target_os = "macos") {
        process::Command::new("open")
            .arg(path)
            .spawn()
            .expect(&format!("cannot open {}", path.to_str().unwrap()));

    } else {
        println!("\n{} open {}", style("info:").green(), path.to_str().unwrap());
    }
}

fn main() {
    //repo-matome need not arguments
    let _ = parse_arg();

    //confirm if there is a path of chrome
    let chrome_path = match browser::default_executable() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("\n{} browser path is {}\n", style("info:").green(), chrome_path.display());

    println!("{}", PROGRAM_MESSAGE);

    println!("{}", SELECT_LANGUAGES);
    println!("{} {}", style("▼ select number 1 ~ 28").cyan(), style(LANG.len()).cyan());
    let lang: String = select_languages();

    println!("\n{}", SELECT_BOUND);
    println!("{}", style("▼ input lower bound").cyan());
    let lower: String = select_lower_bound();

    println!("{}", style("▼ input upper bound").cyan());
    let upper: String = select_uppder_bound();

    if upper != "*".to_string() {
        let l: u64 = lower.parse().unwrap();
        let u: u64 = upper.parse().unwrap();
        if  l >= u {
            eprintln!("error: incorrect range queries...");
            process::exit(1);
        }
    }

    println!("\n{}", SELECT_TOPIC);
    println!("{}", style("▼ input topic words. If you don't need, press enter.").cyan());
    let topic = select_topic();

    println!("\n{}", SELECT_HEADLESS);
    println!("{}", style("▼ Press enter: hide chrome, input `n`: visualize chrome").cyan());
    let headless = select_headless_mode();

    let mut app = App::default()
        .search_lang(&lang)
        .lower_bound(&lower)
        .upper_bound(&upper)
        .topic(&topic)
        .headless_mode(headless)
        .build();

    //search_queries: "language:rust stars:1000..5000 topic:cli  etc...
    app.push_search_queries();

    println!("\n{} {}", style("headless_mode:").green(), app.headless_mode);
    println!("{} {}", style("search query: ").green(), app.search_queries);
    println!("============================================================================");

    setup_environment().expect("cannot set up environment...");
    println!("\n{} setup successfully finished", style("info:").green());
        
    if let Err(e) = scraping_github(&mut app, &chrome_path) {
        eprintln!("{}", e);
        eprintln!("{} try again, `repo-matome`", style("error:").red());
        process::exit(1);
    }

    let check_file_number = check_json_files_in_the_repo_matome_directory();
    create_json_file(&app, check_file_number).expect("cannot create json file");

    println!("{} open {}", style("info:").green(), &INDEX_HTML_PATH.to_str().unwrap());
    thread::sleep(Duration::from_secs(3));
    spawn_index_html(&INDEX_HTML_PATH);
}
