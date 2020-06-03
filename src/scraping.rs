use crate::selector::*;
use crate::app::JsonContents;
use crate::app::App;

use std::path::Path;
use std::thread;
use std::time::Duration;

use serde_json::value;
use headless_chrome::{
    Browser, 
    LaunchOptionsBuilder,
};
use console::style;

pub fn scraping_github(app: &mut App, chrome_abs_path: &Path) -> Result<(), failure::Error> {

    let browser = Browser::new(LaunchOptionsBuilder::default()
        .path(Some(chrome_abs_path.to_path_buf()))
        .headless(app.headless_mode)
        .window_size(Some((1600, 800)))
        .build().unwrap())?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(GITHUB_URL)?;
    //wait for input tag is shown
    tab.wait_for_element(GITHUB_INPUT_SELECTOR)?.click()?;
    //search repo
    tab.type_str(&app.search_queries)?.press_key("Enter")?;

    thread::sleep(Duration::from_secs(2));    

    if let Err(e) = tab.wait_for_element(HIT_REPOSITORY_SELECTOR) {
        eprintln!("\ncannot find repository... match 0");
        return Err(e);
    }

    //scraping 
    let hit_repo = match tab.wait_for_element(HIT_REPOSITORY_SELECTOR)?.call_js_fn("function() { return this.innerText; }", true)?.value.unwrap() {
        value::Value::String(s) => s,
        _ => "unknown".to_string()
    };

    println!("\n{} {}", style("info:").green() , hit_repo);
    println!("----------------------------------------------------------------------------\n");

    let mut json_contents: Vec<JsonContents> = Vec::new();
    let mut target_element;
    
    loop {
        tab.wait_for_element(HIT_REPOSITORY_SELECTOR)?;
        thread::sleep(Duration::from_secs(5));

        target_element = tab.wait_for_elements(LENGTH_OF_ELEMENTS)?;
        println!("{} hit! {} repositories", style("info:").green(), target_element.len());

        app.page_count += 1;
        println!("{} {} page now", style("info:").green(), app.page_count);

        app.total_count += target_element.len() as u64;

        for i in 0..target_element.len() {

            //url
            let url = match tab.wait_for_element(REPO_NAME_AND_HREF_SELECTOR_ARRAY[i])?.call_js_fn("function() { return this.href }", true)?.value.unwrap() {
                value::Value::String(s) => s,
                _ => "NOT_FOUND".to_string()
            };
            println!("{} {}", style("get: ").magenta(), url);

            //repo name
            let repo = match tab.wait_for_element(REPO_NAME_AND_HREF_SELECTOR_ARRAY[i])?.call_js_fn("function() { return this.innerText }", true)?.value.unwrap() {
                value::Value::String(mut s) => {
                    let mut new_s = Vec::new();
                    while let Some(poped) = s.pop() {
                        if poped == '/' {
                            break;
                        }
                        new_s.push(poped);
                    }
                    new_s.reverse();
                
                    let mut repo = String::new();
                    for c in new_s.into_iter() {
                        repo.push(c);
                    }
        
                    repo
                },
                _ => "NOT_FOUND".to_string()
            };


            let desc;
            if let Err(_) = tab.wait_for_element(DESC_SELECTOR_ARRAY[i]) {
                desc = String::from("NOT_FOUND");
            } else {
                desc = match tab.wait_for_element(DESC_SELECTOR_ARRAY[i])?.call_js_fn("function() { return this.innerText }", true)?.value.unwrap() {
                    value::Value::String(s) => s,
                    _ => "NOT_FOUND".to_string()
                };
            }


            let star;
            if let Err(_) = tab.wait_for_element(STAR_SELECTOR_ARRAY[i]) {
                star = String::from("NOT_FOUND");
            } else {
                star = match tab.wait_for_element(STAR_SELECTOR_ARRAY[i])?.call_js_fn("function() { return this.innerText }", true)?.value.unwrap() {
                    value::Value::String(s) => s.trim_start().to_string(),
                    _ => "NOT_FOUND".to_string()
                };
            }


            let keywords;
            if let Err(_) = tab.wait_for_element(KEYWORDS_SELECTOR_ARRAY[i]) {
                keywords = String::from("NOT_FOUND");
            } else {
                keywords = match tab.wait_for_element(KEYWORDS_SELECTOR_ARRAY[i])?.call_js_fn("function() { return this.innerText }", true)?.value.unwrap() {
                    value::Value::String(mut s) => {
                        if s.contains("license") || s.contains("ago") {
                            s.clear();
                            s.push_str("NOT_FOUND");
                        }
    
                        s
                    },
                    _ => "NOT_FOUND".to_string()
                };
            }

            let json_data = JsonContents { url, repo, desc, star, keywords };
            json_contents.push(json_data);
        } //for end

        //check if next button exists
        if let Err(_) = tab.wait_for_element(NEXT_PAGE_BUTTON_SELECTOR) {

            println!("\n{} {} repositories acquired. ($HOME/Downloads/repo-matome-result-dir/xxxxx.json)", style("info:").green(), app.total_count);
            println!("----------------------------------------------------------------------------\n");

            app.contents = Box::new(json_contents);
            println!("{} scraping successfully finished.", style("info:").green());
            break;

        //app.page_count * 10 == contents size
        //if app.page_count is 10, max is 100
        } else if app.page_count >= 10 {

            println!("\n{} {} repositories acquired. ($HOME/Downloads/repo-matome-result-dir/xxxxx.json)", style("info:").green(), app.total_count);
            println!("----------------------------------------------------------------------------\n");

            app.contents = Box::new(json_contents);
            println!("{} The maximum number of items(100) has been reached.", style("info:").green());
            break;
        }
        
        println!("{} go to the next page\n", style("info:").green());
        tab.wait_for_element(NEXT_PAGE_BUTTON_SELECTOR)?.click()?;
    } //loop end
    
    Ok(())
}