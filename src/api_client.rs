use eaglecore_models::Investigation;
use eaglecore_models::Study;

extern crate hyper;
use self::hyper::client::Client;

use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

use std::time::{Duration, SystemTime};

pub struct ApiUser {
    pub id: &'static str,
    pub key: &'static str
}

pub struct ApiClient<'a> {
    pub api_user: &'a ApiUser
}

#[derive(Debug)]
pub enum ApiError {
    Hyper(hyper::Error),
    UnexpectedStatus(String),
    String(String),
    Json(rustc_serialize::json::ParserError),
}

impl From<hyper::Error> for ApiError {
    fn from(err: hyper::Error) -> ApiError {
        ApiError::Hyper(err)
    }
}

const INSTANCE_URL: &'static str = "https://staging.eagle-core.com";

fn build_creds_params(api_user: &ApiUser) -> String {
    format!("client_id={}&client_key={}", api_user.id, api_user.key)
}

fn assert_status(res: &hyper::client::Response, expected_status: &hyper::status::StatusCode) -> Result<(), ApiError> {
    match res.status {
        status if status == *expected_status => Ok(()),
        _ => Err(ApiError::UnexpectedStatus(format!(
            "Got non-ok status: {}", res.status
        )))
    }
}

impl<'a> ApiClient<'a> {
    pub fn list_investigations(&self) -> Result<Vec<Investigation>, ApiError> {
        let client = Client::new();
        let request_url: String = format!(
            "{}/api/v1/investigations?{}",
            INSTANCE_URL, build_creds_params(&self.api_user)
        );
        let mut res = try!(client.get(&request_url).send());
        try!(assert_status(&res, &hyper::Ok));
        let mut res_body = String::new();
        res.read_to_string(&mut res_body);

        Json::from_str(&res_body)
            .map_err(ApiError::Json)
            .and_then(|res_json|
                res_json.as_array().map(|investigation_objs| {
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
                }).ok_or(ApiError::String("Response wasn't an array".to_string()))
            )
    }

    pub fn list_studies_for_investigation(&self, investigation: &Investigation) -> Result<Vec<Study>, ApiError> {
        let client = Client::new();
        let request_url: String = format!(
            "{}/api/v1/investigations/{}/studies?{}",
            INSTANCE_URL, investigation.uuid, build_creds_params(&self.api_user)
        );
        let mut res = try!(client.get(&request_url).send());
        try!(assert_status(&res, &hyper::Ok));

        println!("Reading body for {}...", request_url);
        let start_time = SystemTime::now();
        let mut res_body = String::new();
        let result = res.read_to_string(&mut res_body).unwrap();
        println!(
            "Reading took {} seconds for {} bytes",
            start_time.elapsed().unwrap().as_secs(), result
        );
        let res_json = Json::from_str(&res_body).unwrap();

        let study_objs = res_json.as_array().unwrap();

        Ok(
            study_objs.into_iter().map(|obj_opt| {
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
        )
    }

    pub fn list_studies(&self) -> Result<Vec<Study>, ApiError> {
        let investigations = self.list_investigations();

        investigations.map(|i| {
            i.into_iter().filter(|investigation|
                investigation.name == "Developer test investigation"
            ).flat_map(|investigation| {
                self.list_studies_for_investigation(
                    &investigation
                ).expect(format!(
                    "Unable to fetch studies for investigation {}",
                    investigation.id
                ).as_str())
            } ).collect()
        })
        /*
        Ok(vec!(
            Study { id: 1214, uuid: String::from("018d102a-4887-4ed7-a617-1cec991fdabd"), identifier: String::from("SDH432") },
    Study { id: 1215, uuid: String::from("4781fed1-3364-4e68-8037-184cfe781c5d"), identifier: String::from("TPS-003") },
    Study { id: 1216, uuid: String::from("355c8812-2567-487c-be79-d42785766f1b"), identifier: String::from("SDH204") },
    Study { id: 1213, uuid: String::from("1164e512-42e5-4ea1-85e8-690a52fd061a"), identifier: String::from("SDH034") },
            Study { id: 1520, uuid: String::from("e77088b6-277e-4008-91b3-3f223857696c"), identifier: String::from("E-GEOD-60291") },
            Study { id: 1593, uuid: String::from("f6ab2b7c-b7dc-427b-845a-c20a421f004d"), identifier: String::from("E-MTAB-1065") },
            Study { id: 1541, uuid: String::from("e86749d7-6b6b-40cb-9d78-68872ccd171e"), identifier: String::from("E-GEOD-30450") },
            Study { id: 1603, uuid: String::from("df8b34b8-2bd2-44ab-9083-4f4d5482202c"), identifier: String::from("E-GEOD-30450") },
            Study { id: 1627, uuid: String::from("1151aaae-2601-437b-9985-526c799eef24"), identifier: String::from("E-MTAB-1065") },
            Study { id: 1509, uuid: String::from("dc6864ff-5901-4a4f-b649-d6260602187b"), identifier: String::from("S2") },
            Study { id: 1682, uuid: String::from("cebe7d89-83a0-4fa4-9e4f-a0426f83a671"), identifier: String::from("A1") },
            Study { id: 1683, uuid: String::from("cc72b8e9-9712-4dcc-9772-b6c840f3a77a"), identifier: String::from("STDY009") }
        ))
        */
    }
}
