use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize, Deserialize, Debug)]
struct SessionList {
    status: String,
    sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    session_id: Option<u32>,
    state_id: Option<u32>,
    state_abbr: Option<String>,
    year_start: Option<u32>,
    year_end: Option<u32>,
    prefile: Option<u32>,
    sins_die: Option<u32>,
    special: Option<u32>,
    session_tag: Option<String>,
    session_title: Option<String>,
    session_name: Option<String>,
    dataset_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterList{
    status: Option<String>,
    masterlist: Option<MasterListSession>,
    id: Option<Vec<MasterListBills>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterListSession{
    session_id: Option<u32>,
    state_id: Option<u32>,
    state_abbr: Option<String>,
    year_start: Option<u32>,
    year_end: Option<u32>,
    prefile: Option<u32>,
    sine_die: Option<u32>,
    prior: Option<u32>,
    special: Option<u32>,
    session_tag: Option<String>,
    session_title: Option<String>,
    session_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterListBills{
    bill_id: Option<u32>,
    number: Option<String>,
    change_date: Option<String>,
}

fn load_env() -> String {
    dotenv::dotenv().ok();

    let api_key = dotenv::var("API_KEY").expect("API_KEY must be set");
    return api_key;
}

fn parse_master_list(json_data: &str) -> Result<MasterList, serde_json::Error> {
   let resp: MasterList = serde_json::from_str(json_data)?;
    Ok(resp)
}

fn parse_session_list(json_data: &str) -> Result<SessionList, serde_json::Error> {
   let resp: SessionList = serde_json::from_str(json_data)?;
    Ok(resp)
}

fn parse_session_date(json_data: &str) -> Result<Vec<String>, serde_json::Error> {
   let list = parse_session_list(json_data)?;
   let names = list.sessions.iter().map(|s| s.session_name.clone().unwrap_or_default()).collect::<Vec<String>>();
   Ok(names)
}

#[get("/masterlist")]
async fn get_master_list() -> actix_web::Result<HttpResponse> {
    // Fetch remote body. Map reqwest errors into Actix internal errors so Actix can
    // return 500s.
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com
/?key={}&op=getMasterList&state=IN&year=2024", api_key)).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    parse_master_list(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;
    // Return the fetched body to the browser. Set a content type if you know the format.
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(resp_text))
}

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/master_list")]
async fn get_masterlist() -> actix_web::Result<HttpResponse> {
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com/?key={}&op=getMasterListRaw&id=2143", api_key)).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        parse_master_list(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the fetched body to the browser. Set a content type if you know the format.
        Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(resp_text))
}

#[get("/sessions")]
async fn get_session_list() -> actix_web::Result<HttpResponse> {
    // Fetch remote body. Map reqwest errors into Actix internal errors so Actix can return 500s.
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com/?key={}&op=getSessionList&state=IN", api_key)).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    parse_session_list(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the fetched body to the browser. Set a content type if you know the format.
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(resp_text))
}

#[get("/sessions_name")]
async fn get_session_names() -> actix_web::Result<HttpResponse> {
    // Fetch remote body. Map reqwest errors into Actix internal errors so Actix can return 500s.
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com/?key={}&op=getSessionList&state=IN", api_key)).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let names = parse_session_date(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the fetched body to the browser. Set a content type if you know the format.
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(names.join(", ")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
          .wrap(Cors::permissive()) // Enable CORS with permissive settings
          .service(greet)
          .service(get_session_list)
          .service(get_session_names)
          .service(get_masterlist)
    })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}