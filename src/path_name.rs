use std::path::PathBuf;

lazy_static! {
    pub static ref REPO_MATOME_DIR_PATH: PathBuf = {
        let mut download_dir = dirs::download_dir().unwrap();
        download_dir.push("repo-matome-result-dir");
        download_dir      
    };
}

lazy_static! {
    pub static ref INDEX_HTML_PATH: PathBuf = {
        let mut index_html_path = dirs::download_dir().unwrap();
        index_html_path.push("repo-matome-result-dir");
        index_html_path.push("index.html");
        index_html_path
    };
}