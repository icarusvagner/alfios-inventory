#[derive(Debug)]
pub struct LoginProvider {
    pub username: String,
    pub password: String,
}

impl LoginProvider {
    pub fn set_login(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;
    }

    pub fn check_login(&self) -> bool {
        let username = String::from("admin123");
        let password = String::from("admin123");

        self.username.eq(&username) && self.password.eq(&password)
    }
}
