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


impl<'a> ApiClient<'a> {
    pub fn list_investigations(&self) -> Vec<Investigation> {
        let client = Client::new();

        let request_url: String = format!(
            "http://staging.eagle-core.com/api/v1/investigations?client_id={}&client_key={}",
            self.api_user.id, self.api_user.key
        );
        let mut res = client.get(&request_url).send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut res_body = String::new();
        res.read_to_string(&mut res_body);

        println!("{}",res_body);

        let res_json = Json::from_str(&res_body).unwrap();
        let investigation_objs = res_json.as_array();

        let results: Vec<Investigation> = investigation_objs.unwrap().into_iter()
            .map( |obj|
                Investigation {
                    id: 1,
                    name: String::from(
                        obj.as_object().unwrap().get("title").unwrap().as_string().unwrap()
                    )
                }
            ).collect();
       results 
    }

    pub fn list_studies_for_investigation(&self, investigation: &Investigation) -> Vec<Study> {
        vec!(
            Study { id: 5, identifier: format!("Study in {}", investigation.name)}
        )
    }
}
