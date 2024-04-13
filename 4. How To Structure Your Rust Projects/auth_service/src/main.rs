use auth_service777777777::Credentials;

fn main() {
    let creds = Credentials {
        username: "Irfan".to_owned(),
        password: "password".to_owned(),
    };

    auth_service777777777::authenticate(creds);
}
