const URL: &str = "https://www.rightmove.co.uk";
const JOKE: &str = "https://official-joke-api.appspot.com/random_joke";

#[test]
fn test_ping_rightmove() {
    let response = match ureq::get(URL).call() {
        Ok(response) => response,
        Err(err) => panic!("request failed, {err}"),
    };
    dbg!("{}", response.status());

    assert_eq!(response.status(), 200);
}

#[test]
fn test_read_rightmove_html() {
    let mut response = match ureq::get(JOKE).call() {
        Ok(response) => response,
        Err(err) => panic!("request failed, {err}"),
    };

    let body = response.body_mut();

    dbg!("{}", &body);

    let body_str = match body.read_to_string() {
        Ok(body_str) => body_str,
        Err(e) => panic!("{}", e),
    };
    dbg!("hello tests");
    dbg!("{}", body_str);

    assert_eq!(1, 1);
}
