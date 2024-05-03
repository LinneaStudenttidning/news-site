#[derive(FromForm)]
pub struct Login<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(FromForm)]
pub struct ChangePasswordSelf<'a> {
    #[field(name = "current_password")]
    pub current_password: &'a str,
    #[field(name = "new_password")]
    pub new_password: &'a str,
    #[field(name = "confirm_new_password-password")]
    pub confirm_new_password: &'a str,
}

#[derive(FromForm)]
pub struct ChangePasswordOther<'a> {
    pub username: &'a str,
    #[field(name = "new-password")]
    pub new_password: &'a str,
}
