

#[derive(Debug)]
pub enum UiToCore {
    Exit {
        user_initiated: bool,
    }
}
