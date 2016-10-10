pub mod eaglecore_models;
pub mod api_client;

fn main() {
    let api_user = api_client::ApiUser {
        id: "eagletest1@eaglegenomics.com",
        key: "eagletest1@eaglegenomics.com",
    };

    let api_client = api_client::ApiClient { api_user: &api_user };
    let investigations = api_client.list_investigations();

    /*
    let all_studies = investigations.into_iter().flat_map(|investigation|
        let studies = api_client.list_studies_for_investigation(&investigation);
        studies
    );
    */
    for investigation in investigations {
        println!("{}", investigation.name);
        let studies = api_client.list_studies_for_investigation(&investigation);
        for study in studies {
            println!("\t{}", study.identifier)
        }
    }
}
