pub fn login(creds: models::Credentials) {
    // println!("Logging in with username: {}", creds.username);
    crate::database::get_user();
}

fn logout() {
    // println!("Logging out");
}

pub mod models;
