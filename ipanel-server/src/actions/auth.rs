use ipanel_services::ServiceManager;

pub async fn get_auth_methods(
    _services: &ServiceManager,
    domain: &str,
    username: &str,
) -> Vec<String> {
    vec![format!("{domain}/{username}")]
}
