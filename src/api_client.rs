use eaglecore_models::Investigation;
use eaglecore_models::Study;

extern crate hyper;
use self::hyper::client::Client;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

pub struct ApiUser {
    pub id: &'static str,
    pub key: &'static str
}

pub struct ApiClient<'a> {
    pub api_user: &'a ApiUser
}

const INSTANCE_URL: &'static str = "https://staging.eagle-core.com";

fn build_creds_params(api_user: &ApiUser) -> String {
    format!("client_id={}&client_key={}", api_user.id, api_user.key)
}

impl<'a> ApiClient<'a> {
    pub fn list_investigations(&self) -> Vec<Investigation> {
        let client = Client::new();
        let request_url: String = format!(
            "{}/api/v1/investigations?{}",
            INSTANCE_URL, build_creds_params(&self.api_user)
        );
        let mut res = client.get(&request_url).send().unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut res_body = String::new();
        res.read_to_string(&mut res_body);

        let res_json = Json::from_str(&res_body).unwrap();
        let investigation_objs = res_json.as_array().unwrap();

        investigation_objs.into_iter()
            .map( |obj_opt| {
                let obj = obj_opt.as_object().unwrap();
                Investigation {
                    id: obj.get("id").unwrap().as_u64().unwrap(),
                    uuid: String::from(
                        obj.get("uuid").unwrap().as_string().unwrap()
                    ),
                    name: String::from(
                        obj.get("title").unwrap().as_string().unwrap()
                    )
                }
            } ).collect()
    }

    pub fn list_studies_for_investigation(&self, investigation: &Investigation) -> Vec<Study> {
        let client = Client::new();
        let request_url: String = format!(
            "{}/api/v1/investigations/{}/studies?{}",
            INSTANCE_URL, investigation.uuid, build_creds_params(&self.api_user)
        );
        let mut res = client.get(&request_url).send().unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut res_body = String::new();
        let result = res.read_to_string(&mut res_body).unwrap();
        let res_json = Json::from_str(&res_body).unwrap();

        let study_objs = res_json.as_array().unwrap();

        study_objs.into_iter()
            .map( |obj_opt| {
                let obj = obj_opt.as_object().unwrap();
                Study {
                    id: obj.get("id").unwrap().as_u64().unwrap(),
                    uuid: String::from(
                        obj.get("uuid").unwrap().as_string().unwrap()
                    ),
                    identifier: String::from(
                        obj.get("identifier").unwrap().as_string().unwrap()
                    )
                }
            } ).collect()
    }

    pub fn list_studies(&self) -> Vec<Study> {
        let investigations: Vec<Investigation> = self.list_investigations();

        investigations.into_iter().flat_map(|investigation: Investigation| {
            let studies = self.list_studies_for_investigation(
                &investigation
            );
            println!("Fetching studies of investigation {}", investigation.id);
            studies
        } ).collect()
    }
}
