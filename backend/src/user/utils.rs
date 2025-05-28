pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub verification_token: String, // Add this field
}
