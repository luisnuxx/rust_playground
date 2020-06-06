
// extern crate reqwest;
/*
use self::reqwest::Error;


#[get("/http_test")]
pub async fn http_test() -> Result<String,Error> {

    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    match body {
        Ok(v) =>  Ok( format!("body = {:?}", body) ),
        Err(e) => Err ( format!("error parsing header: {:?}", e).as_str() ),
    }

}
*/
