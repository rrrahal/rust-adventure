use reqwest::header::USER_AGENT;

fn main() {
    let api_token = std::env::var("API_TOKEN")
        .expect("to be a token");

    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .header(USER_AGENT, "rafael")
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("the body to be json");

    dbg!(response);

}
