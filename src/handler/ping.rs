#[get("/")]
pub fn ping() -> String {
    "hello world".to_owned()
}
