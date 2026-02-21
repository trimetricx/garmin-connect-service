#[cfg(test)]
mod user_repository_tests {
    use garmin_connect_service::garmin_oauth::persistance::user_repository::UserRepository;
    #[test]
    pub fn test_add_text() {
        let user_repository: UserRepository = UserRepository {};
        assert_eq!(3, user_repository.add_text(3));
    }
}
