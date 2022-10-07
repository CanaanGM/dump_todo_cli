pub struct Base{
    // pub id : i8, // later
    pub title: String,
    pub status: String
}

impl Base{
    pub fn new (
        input_title: &str,
        input_status: &str
    ) -> Base {
        return Base {
            title: input_title.to_string(),
            status: input_status.to_string()
        }
    }
}