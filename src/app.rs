use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct JsonContents {
    pub url: String,
    pub repo: String,
    pub desc: String,
    pub star: String,
    pub keywords: String
}

#[derive(Debug)]
pub struct App { 
    pub headless_mode: bool,
    pub page_count: u64,
    pub total_count: u64,
    pub search_lang: String,
    pub lower_bound: String,
    pub upper_bound: String,
    pub topic: String,
    pub search_queries: String,
    /// collect json data
    pub contents: Box<Vec<JsonContents>>
}

impl Default for App {
    fn default() -> Self {
        Self {
            headless_mode: true,
            page_count: 0,
            total_count: 0,
            search_lang: String::new(),
            lower_bound: String::new(),
            upper_bound: String::new(),
            topic: String::new(),
            search_queries: String::new(),
            contents: Box::new(Vec::new())
        }
    }
}

impl App {
    pub fn search_lang(&mut self, lang: &str) -> &mut App {
        self.search_lang = lang.to_string();
        self
    }

    pub fn lower_bound(&mut self, lower: &str) -> &mut App {
        self.lower_bound = lower.to_string();
        self
    }

    pub fn upper_bound(&mut self, upper: &str) -> &mut App {
        self.upper_bound = upper.to_string();
        self
    }

    pub fn topic(&mut self, topic: &str) ->  &mut App {
        self.topic = topic.to_string();
        self
    }

    pub fn headless_mode(&mut self, headless: bool) -> &mut App {
        self.headless_mode = headless;
        self
    }

    pub fn build(&self) -> Self {
        Self {
            headless_mode: self.headless_mode,
            search_lang: self.search_lang.clone(),
            lower_bound: self.lower_bound.clone(),
            upper_bound: self.upper_bound.clone(),
            topic: self.topic.clone(),
            ..App::default()
        }
    }
}

impl App {
    pub fn push_search_queries(&mut self) {
        if self.topic == "".to_string() {
            self.search_queries = format!("language:{} stars:{}..{}",
                self.search_lang, self.lower_bound, self.upper_bound
            );
        } else {

            let split_string: Vec<&str> = self.topic.split(' ').collect();
            
            let queries = match split_string.len() {
                1 => format!("language:{} stars:{}..{} topic:{}", self.search_lang, self.lower_bound, self.upper_bound, self.topic),
                _ => format!("language:{} stars:{}..{} topic:\"{}\"", self.search_lang, self.lower_bound, self.upper_bound, self.topic),
            };

            self.search_queries = queries;
        }
    }
}