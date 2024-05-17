use crate::errors::Error;

#[allow(unused)]
pub struct UserModel {
    id: Option<u64>,
    username: String,
    email: String,
    avatar_url: Option<String>,
    phone_number: Option<String>,
    status: i8,
    password: String,
    // created_at: String,
    // updated_at: String,
}

impl UserModel {
    pub fn general_create<E, U, PH>(email: E, username: U, password_hash: PH) -> Self
    where
        E: Into<String>,
        U: Into<String>,
        PH: Into<String>,
    {
        Self {
            id: None,
            username: username.into(),
            email: email.into(),
            avatar_url: None,
            phone_number: None,
            status: 0,
            password: password_hash.into(),
            // created_at: ,
            // updated_at: (),
        }
    }

    // TODO: hash password for new user or update user
    #[allow(unused)]
    pub fn hash_password<P>(password: P) -> Result<String, Error>
    where
        P: AsRef<str> + Send + 'static
    {
        unimplemented!()
    }
}
