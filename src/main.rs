pub mod eaglecore_models;
pub mod api_client;

fn main() {
    let api_user = api_client::ApiUser {
        id: "eagletest1@eaglegenomics.com",
        key: "cd209ffe-ac7d",
    };

    let api_client = api_client::ApiClient { api_user: &api_user };
    let all_studies = api_client.list_studies();

    for study in all_studies {
        println!(
            "Study {{ id: {}, uuid: String::from(\"{}\"), identifier: String::from(\"{}\") }}",
            study.id, study.uuid, study.identifier
        )
    }
}
