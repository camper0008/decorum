use salvo::oapi::extract::QueryParam;

#[salvo::endpoint]
pub async fn route(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}
