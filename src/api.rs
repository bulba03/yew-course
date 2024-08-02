use std::ops::Deref;

use gloo_console::log;
use gloo_net::{ http::{ Request, Response }, Error };
use http_auth_basic::Credentials;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use yew::use_state;
use yewdux::{dispatch, use_dispatch, use_store};

use crate::app::{ Role, SERVER_URL};

#[derive(Serialize, Deserialize)]
pub struct ApiRegisterResponse {
    pub token: String,
}
#[derive(Serialize, Deserialize)]
struct ApiRegisterResponseData {
    pub data: ApiRegisterResponse,
}
#[derive(Deserialize, Serialize)]
pub struct GetCourseBody {
    course_id: i128,
    is_all: bool,
}
#[derive(Serialize,Deserialize, Clone)]
pub struct AuthResp {
    pub token: String,
    pub user_id: i32,
    pub role: Role
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ApiCourseInfoData {
    pub id: i32,
    pub name: String,
    pub short_desc: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub teacher_id: i32
}

pub async fn api_register(
    first_name: String,
    last_name: String,
    password: String,
    email: String
) -> bool {
    let mut route = SERVER_URL.to_string();
    route.push_str("/users/create");
    let body =
        json!({
        "email": email,
        "password": password,
        "first_name": first_name,
        "last_name": last_name,
        "avatar_type": 1
      });
    let responce = Request::post(&route)
        .header("content-type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send().await
        .unwrap();
    log!(responce.status());
    responce.ok()
    // responce.data
}

pub async fn api_login(email: String, password: String) -> Result<AuthResp, Error> {
    let mut route = SERVER_URL.to_string();
    let credentials = Credentials::new(&email, &password);
    route.push_str("/users/auth");
    let responce = Request::post(&route)
        .header("Authorization", credentials.as_http_header().as_str())
        .send().await;
    if responce.is_ok() {
        let token = responce.unwrap().json::<AuthResp>().await.unwrap();
        Ok(token)
    } else {
        Err(Error::GlooError("SS".to_string()))
    }
}

pub async fn api_get_course_info(id: i128) -> ApiCourseInfoData {
    let mut route = SERVER_URL.to_string();
    route.push_str("/courses/get/");
    route.push_str(id.to_string().as_str());
    let responce = Request::get(&route)
        .send().await
        .unwrap().json::<Vec<ApiCourseInfoData>>().await.unwrap();
    responce.get((id - 1) as usize).unwrap().clone()
    // todo!()
}
pub async fn api_get_courses() -> Vec<ApiCourseInfoData> {
    let mut route = SERVER_URL.to_string();
    route.push_str("/courses/get/");
    route.push_str("0");
    let responce = Request::get(&route)
        .send().await
        .unwrap().json::<Vec<ApiCourseInfoData>>().await.unwrap();
    responce
} 

pub async fn api_get_users() -> Vec<User> {
    let mut route = SERVER_URL.to_string();
    route.push_str("/users/all");
    Request::get(&route).send().await.unwrap().json::<Vec<User>>().await.unwrap()
}
pub async fn api_change_course_info(name: String, desc: String, short_desc: String, image: String, id: i128) {
    let mut route = SERVER_URL.to_string();
    route.push_str("/courses/edit");
    let body= json!({"name": name, "description": desc, "short_desc": short_desc, "image_url": image, "course_id": id});
    let responce = Request::post(&route)
    .header("content-type", "application/json")
    .header("Authorization", "SAASA")
    .body(body.to_string()).unwrap()
        .send().await;
}