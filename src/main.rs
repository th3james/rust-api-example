pub mod eaglecore_models;
pub mod api_client;

use std::fs;

fn main() {
    let api_user = api_client::ApiUser {
        id: "eagletest1@eaglegenomics.com",
        key: "cd209ffe-ac7d",
    };

    let api_client = api_client::ApiClient { api_user: &api_user };
    let all_studies = api_client.list_studies();

    let candidate_file_paths: Vec<std::path::PathBuf> = fs::read_dir("files-to-upload").unwrap().map( |dir|
        dir.unwrap()
    ).filter( |dir|
        dir.file_type().unwrap().is_file() &&
            !dir.file_name().into_string().unwrap().starts_with(".")
    ).map( |dir|
        dir.path()
    ).collect();
    
    for path in candidate_file_paths {
        println!("{}", path.to_str().unwrap());
    }
}
