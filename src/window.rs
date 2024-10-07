use macroquad::color::Color;
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub clear_color: Color,
}

impl Window {
    //Returns a new window object which contains the properties a window must use
    pub fn new(width: f32, height: f32, clear_color: Color) -> Self {
        Window {
            width,
            height,
            clear_color,
        }
    }
}
