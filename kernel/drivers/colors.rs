#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Orange,
}

pub fn get_color(color: Color) -> u32 {
    match color {
        Color::Black => 0xFF000000,
        Color::White => 0xFFFFFFFF,
        Color::Red => 0xFFFF0000,
        Color::Green => 0xFF00FF00,
        Color::Blue => 0xFF0000FF,
        Color::Yellow => 0xFFFFFF00,
        Color::Magenta => 0xFFFF00FF,
        Color::Cyan => 0xFF00FFFF,
        Color::Orange => 0xFFAB5130,
    }
}
