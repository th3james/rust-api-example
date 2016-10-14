pub mod eaglecore_models;
pub mod api_client;

use std::fs;
use std::path::PathBuf;

fn main() {
    let api_user = api_client::ApiUser {
        id: "eagletest1@eaglegenomics.com",
        key: "cd209ffe-ac7d",
    };

    let api_client = api_client::ApiClient { api_user: &api_user };
    let all_studies = api_client.list_studies();

    let candidate_file_paths: Vec<PathBuf> = fs::read_dir(
        "files-to-upload"
    ).unwrap().map( |dir|
        dir.expect("Dir couldn't be opened")
    ).filter( |dir|
        dir.file_type().expect("Filetype unreadable").is_file() &&
            !dir.file_name().into_string().expect("Can't read filename")
                .starts_with(".")
    ).map( |dir|
        dir.path()
    ).collect();

    for ref path in candidate_file_paths.iter() {
        println!("{}", path.to_str().unwrap());
    }

    let upload_pairs: Vec<(&eaglecore_models::Study, &PathBuf)> =
        candidate_file_paths.iter().map( |path| {
            let file_name = path.file_name().and_then(|os_str|
                                os_str.to_str()
                             ).expect("Couldn't unwrap filename");
            let id_prefix = file_name.split_whitespace().next()
                .expect("Can't split whitespace");
            let study = all_studies.iter().find( |study| {
                study.identifier == id_prefix
            }).expect(
                format!("Couldn't find a study for {}", id_prefix).as_str()
            );
            (study, path)
        }).collect();
}
