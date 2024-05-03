#[derive(Debug, FromForm)]
pub struct EditAboutUs<'a> {
    #[field(name = "about-us")]
    pub about_us: &'a str,
}
