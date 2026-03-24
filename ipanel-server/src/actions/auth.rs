use ipanel_services::ServiceManager;

pub async fn get_auth_methods(_services: &ServiceManager, _username: &str) -> Vec<String> {
    vec!["otp".to_string()]
}
