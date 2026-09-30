use ratatui::{
    Frame,
    layout::{Layout, Rect, Constraint, Flex},
};

pub fn getCenterArea(frame: &Frame, rect: &Rect) -> Rect {
    let [verticalArea] = Layout::vertical([Constraint::Length(rect.height)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Length(rect.width)])
        .flex(Flex::Center)
        .areas(verticalArea);
    return area;
}

pub fn splitItemAndPrice(itemAndPrice: &str) -> (&str, f32) {
    if let Some((itemPart, pricePart)) = itemAndPrice.rsplit_once('-') {
        if let Ok(price) = pricePart.trim().trim_start_matches('£').trim().parse::<f32>() {
            return (itemPart.trim(), price);
        }
    }

    return (itemAndPrice, 0.0);
}
#[test]
fn testSplitItemAndPrice() {
    assert_eq!(splitItemAndPrice("Keyboard - £15"), ("Keyboard", 15.0));
    assert_eq!(splitItemAndPrice(" Mouse-And-Keyboard   -  £     45"), ("Mouse-And-Keyboard", 45.0));
    assert_eq!(splitItemAndPrice(" Headset-£ 20"), ("Headset", 20.0));
    assert_eq!(splitItemAndPrice("Invalid Line"), ("Invalid Line", 0.0));
}
