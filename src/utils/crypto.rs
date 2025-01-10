use bcrypt;
pub fn hash_password(password: String) -> String {
    let salt = bcrypt::DEFAULT_COST;
    let hashed_password = bcrypt::hash(password, salt).expect("Failed to hash password");
    hashed_password
}

pub fn hash_compare(password1: &str, password2: &str) -> bool {
    bcrypt::verify(password1, password2).unwrap()
}
