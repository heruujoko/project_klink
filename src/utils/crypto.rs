use bcrypt;
pub fn hash_password(password: String) -> String {
    let salt = bcrypt::DEFAULT_COST;
    let hashed_password = bcrypt::hash(password, salt).expect("Failed to hash password");
    hashed_password
}
