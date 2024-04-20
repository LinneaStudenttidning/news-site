#[derive(Debug, FromForm)]
pub struct NewCreator<'a> {
    pub username: &'a str,
    #[field(name = "display-name")]
    pub display_name: &'a str,
    pub password: &'a str,
    #[field(name = "as-publisher")]
    pub as_publisher: bool,
}

#[derive(Debug, FromForm)]
pub struct OnlyUsername<'a> {
    pub username: &'a str,
}

#[derive(Debug, FromForm)]
pub struct UpdateProfile<'a> {
    #[field(name = "display-name")]
    pub display_name: Option<&'a str>,
    pub biography: Option<&'a str>,
}
