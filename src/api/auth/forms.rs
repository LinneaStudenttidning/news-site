#[derive(FromForm)]
pub struct Login<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub referer: Option<&'a str>,
}

#[derive(FromForm)]
pub struct ChangePasswordSelf<'a> {
    #[field(name = "current-password")]
    pub current_password: &'a str,
    #[field(name = "new-password")]
    pub new_password: &'a str,
    #[field(name = "confirm-new-password")]
    pub confirm_new_password: &'a str,
}

#[derive(FromForm)]
pub struct ChangePasswordOther<'a> {
    pub username: &'a str,
    #[field(name = "new-password")]
    pub new_password: &'a str,
}
