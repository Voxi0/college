use ratatui::{
    Frame,
    layout::{Layout, Rect, Direction, Constraint, Flex},
};

pub fn getCenterArea(frame: &Frame, width: u16, height: u16) -> Rect {
    let [verticalArea] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(verticalArea);

    return area;
}
