use std::error::Error;
use std::fmt::Display;
use url::{Url, ParseError};
use reqwest::header;

pub mod config;
pub mod issue;
pub mod worklog;
use worklog::WorklogResponse;


const ISSUE_PATH: &str = "rest/api/2/issue";
const ISSUE_KEY_PATH: &str = "rest/api/2/issue/{key}";
const WORKLOG_PATH: &str = "rest/com.deniz.jira.worklog/1.0/worklog";
const TIMESHEET_PATH: &str = "rest/com.deniz.jira.worklog/1.0/timesheet/user";


#[derive(Debug)]
pub enum ClientErr {
    ReqwestErr(reqwest::Error),
    ParseErr(ParseError),
    HttpErr(HttpError),
}

impl Display for ClientErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientErr::ReqwestErr(reqwest_error) => 
                write!(f, "{}", reqwest_error),
            ClientErr::ParseErr(parse_error) => 
                write!(f, "{}", parse_error),
            ClientErr::HttpErr(http_error) =>
                write!(f, "{}", http_error),
        }
    }
}

impl Error for ClientErr {}

impl From<ParseError> for ClientErr {
    fn from(err: ParseError) -> Self {
        ClientErr::ParseErr(err)
    }
}

impl From<reqwest::Error> for ClientErr {
    fn from(err: reqwest::Error) -> Self {
        ClientErr::ReqwestErr(err)
    }
}

impl From<HttpError> for ClientErr {
    fn from(err: HttpError) -> Self {
        ClientErr::HttpErr(err)
    }
}

#[derive(Debug)]
pub struct HttpError {
    status: reqwest::StatusCode,
    text: String,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Http Error: status code {} response {}", self.status, self.text)
    }
}



#[derive(Debug)]
struct Urls {
    url: Url,
}

impl Urls {
    fn new(url: &str) -> Result<Urls, ParseError> {
        Ok(Urls { url: Url::parse(url)? })
    }
    fn issue_by_key(&self, key: &str) -> Result<Url, ParseError> {
        let request_url = format!("/{issue}/{key}", issue = ISSUE_PATH, key = key); // эту часть надо делать без format только join
        Ok(self.url.join(&request_url)?)
    }
    fn issue(&self) -> Result<Url, ParseError> {
        Ok(self.url.join(&ISSUE_PATH)?)
    }
    fn worklog_by_dates(&self, start_date: &str, end_date: &str, target_key: &str) -> Result<Url, ParseError> {
        let mut request = self.url.join(&TIMESHEET_PATH)?;
        // startDate={startDate}&endDate={endDate}&targetKey={targetKey}
        request.query_pairs_mut()
            .append_pair("startDate", start_date)
            .append_pair("endDate", end_date)
            .append_pair("targetKey", target_key);

        Ok(request)
    }

}

#[derive(Debug)]
struct Request {
    client: reqwest::Client,
}

impl Request {
    fn headers() -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
        return headers;
    }
    fn new() -> Result<Request, reqwest::Error> {
        let client = reqwest::Client::builder()
        .default_headers(Request::headers())
        .build()?;

        Ok(Request { client: client })
    }
    async fn get(&self, url: &str, username: &str, password: &str) -> Result<reqwest::Response, reqwest::Error> {
        let resp = self.client.get(url)
        .basic_auth(username, Some(password))
        .send()
        .await?;
        eprintln!("Response: {:?} {}", resp.version(), resp.status());
        Ok(resp)
    }

    
}


#[derive(Debug)]
pub struct Client<'a> {
    client: Request,
    url: Urls,
    username: &'a str,
    password: &'a str,
}

impl<'a> Client<'a> {
    pub fn build(credits: (&'a str, &'a str, &'a str)) -> Result<Client<'a>, ClientErr> {

        let client = Request::new()?;
        let url = Urls::new(credits.0)?;

        Ok(Client { 
            client,
            url,
            username: credits.1,
            password: credits.2,
         })
    }


    pub async fn get_worklogs(&self, start_date: &str, end_date: &str) -> Result<WorklogResponse, ClientErr> {
        let request_url = self.url.worklog_by_dates(start_date, end_date, &self.username)?;
        let resp = self.client.get(request_url.as_str(), &self.username, &self.password).await?;
        let status = resp.status();
        if status.is_success() { // TODO переписать бы это на макрос или еще как то 
            Ok(resp.json::<WorklogResponse>().await?)
        } else {
            let text = resp.text().await?;
            Err(ClientErr::HttpErr(HttpError {status, text}))
        }

        
    }

    // pub async fn get_issue_by_key(&self, key: &str) -> Result<WorklogResponse, ClientErr> {
    //     let request_url = self.url.issue_by_key(key)?;
    //     Ok(self.client.get_issue_by_key(request_url.as_str(), &self.username, &self.password).await?)
    // }
}