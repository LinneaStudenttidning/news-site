#[derive(FromForm)]
pub struct Login<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(FromForm)]
pub struct ChangePasswordSelf<'a> {
    pub current_password: &'a str,
    pub new_password: &'a str,
    pub confirm_new_password: &'a str,
}

#[derive(FromForm)]
pub struct ChangePasswordOther<'a> {
    pub username: &'a str,
    #[field(name = "new-password")]
    pub new_password: &'a str,
}
