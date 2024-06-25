use axum::http::HeaderMap;

pub async fn mirror_user_agents(headers: HeaderMap) -> String {
  // let my_header = headers.get("my-header").unwrap().to_str().unwrap_or_default().to_string();

  let user_agents = headers.get("user-agent").unwrap().to_str().unwrap().to_owned();

  user_agents
}