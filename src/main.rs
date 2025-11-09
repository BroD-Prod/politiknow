use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::{ Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize, Deserialize, Debug)]
struct SessionList {
    status: String,
    sessions: Vec<Session>
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
    dataset_hash: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterList{
    status: Option<String>,
    masterlist: Option<MasterListSession>,
    id: Option<Vec<MasterListBills>>
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
    session_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterListBills{
    bill_id: Option<u32>,
    number: Option<String>,
    change_date: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Bill{
    status: Option<String>,
    bill: Option<BillInner>,
    url: Option<String>,
    state_link: Option<String>,
    completed: Option<u32>,
    status_code: Option<u32>,
    status_date: Option<String>,
    progress: Option<Vec<ProgressDatesInner>>,
    state: Option<String>,
    state_id: Option<u32>,
    bill_number: Option<String>,
    bill_type: Option<String>,
    bill_type_id: Option<u32>,
    body: Option<String>,
    body_id: Option<u32>,
    current_body: Option<String>,
    current_body_id: Option<u32>,
    title: Option<String>,
    description: Option<String>,
    pending_committee_id: Option<u32>,
    committee: Option<Vec<String>>,
    referrals: Option<Vec<String>>,
    history: Option<Vec<HistoryInner>>,
    sponsors: Option<Vec<SponsorInner>>,
    sasts: Option<Vec<String>>,
    subjects: Option<Vec<String>>,
    texts: Option<Vec<TextInner>>,
    votes: Option<Vec<String>>,
    amendments: Option<Vec<String>>,
    supplements: Option<Vec<String>>,
    calendar: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
struct 
BillInner{
    bill_id: Option<u32>,
    change_hash: Option<String>,
    session_id: Option<u32>,
    session: Option<BillSessionInner>
}

#[derive(Serialize, Deserialize, Debug)]
struct BillSessionInner{
    session_id: Option<u32>,
    state_id: Option<u32>,
    year_start: Option<u32>,
    year_end: Option<u32>,
    prefile: Option<u32>,
    sine_die: Option<u32>,
    prior: Option<u32>,
    special: Option<u32>,
    session_tag: Option<String>,
    session_title: Option<String>,
    session_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct ProgressDatesInner{
    date: Option<String>,
    event: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryInner{
    date: Option<String>,
    action: Option<String>,
    chamber: Option<String>,
    chamber_id: Option<u32>,
    importance: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SponsorInner{
    people_id: Option<u32>,
    people_hash: Option<String>,
    party_id: Option<u32>,
    party: Option<String>,
    name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    suffix: Option<String>,
    nickname: Option<String>,
    district: Option<String>,
    ftm_eid: Option<u32>,
    votesmart_id: Option<u32>,
    opensecrets_id: Option<String>,
    knowwho_pid: Option<u32>,
    ballotpedia: Option<String>,
    sponsor_type_id: Option<u32>,
    sponsor_order: Option<u32>,
    commitee_sponsor: Option<u32>,
    committee_id: Option<u32>,
    state_federal: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct TextInner{
    doc_id: Option<u32>,
    date: Option<String>,
    r#type: Option<String>,
    type_id: Option<u32>,
    mime: Option<String>,
    mime_id: Option<u32>,
    url: Option<String>,
    state_link: Option<String>,
    text_size: Option<u32>,
    text_hash: Option<String>,
    alt_bill_text: Option<String>,
    alt_mime: Option<String>,
    alt_mime_id: Option<u32>,
    alt_state_link: Option<String>,
    alt_text_size: Option<u32>,
    alt_text_hash: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct PersonOuter{
    status: Option<String>,
    person: Option<PersonInner>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PersonInner{
    people_id: Option<u32>,
    person_hash: Option<String>,
    party_id: Option<String>,
    state_id: Option<u32>,
    party: Option<String>,
    role_id: Option<u32>,
    role: Option<String>,
    name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    suffix: Option<String>,
    nickname: Option<String>,
    district: Option<String>,
    ftm_eid: Option<u32>,
    votesmart_id: Option<u32>,
    opensecrets_id: Option<String>,
    knowwho_pid: Option<u32>,
    ballotpedia: Option<String>,
    bioguide_id: Option<String>,
    committee_sponsor: Option<u32>,
    committee_id: Option<u32>,
    state_federal: Option<u32>
}

fn load_env() -> String {
    dotenv::dotenv().ok();

    let api_key = dotenv::var("API_KEY").expect("API_KEY must be set");
    return api_key;
}

fn parse_bill(json_data: &str) -> Result<Bill, serde_json::Error> {
    let resp: Bill = serde_json::from_str(json_data)?;
     Ok(resp)
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

fn parse_person(json_data: &str) -> Result<PersonOuter, serde_json::Error> {
    let resp: PersonOuter = serde_json::from_str(json_data)?;
     Ok(resp)
}


#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("get_bill/{bill_id}")]
async fn get_bill(bill_id: web::Path<u32>) -> actix_web::Result<HttpResponse>{
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com/?key={}&op=getBill&id={}", api_key, bill_id)).await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        parse_bill(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(resp_text))
}

#[get("/master_list_raw")]
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

#[get("/person/{people_id}")]
async fn get_person(people_id: web::Path<u32>) -> actix_web::Result<HttpResponse>{
    let api_key = load_env();
    let resp_text = reqwest::get(format!("https://api.legiscan.com/?key={}&op=getPerson&id={}", api_key, people_id))
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    parse_person(&resp_text).map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(resp_text))
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
          .service(get_bill)
          .service(get_person)
    })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}