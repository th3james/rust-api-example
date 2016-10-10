use eaglecore_models::Investigation;
use eaglecore_models::Study;

pub struct ApiUser {
    pub id: &'static str,
    pub key: &'static str
}

pub struct ApiClient<'a> {
    pub api_user: &'a ApiUser
}


impl<'a> ApiClient<'a> {
    pub fn list_investigations(&self) -> Vec<Investigation> {
        vec!(
            Investigation { id: 1, name: String::from("Investigation A") }
        )
    }

    pub fn list_studies_for_investigation(&self, investigation: &Investigation) -> Vec<Study> {
        vec!(
            Study { id: 5, identifier: format!("Study in {}", investigation.name)}
        )
    }
}
