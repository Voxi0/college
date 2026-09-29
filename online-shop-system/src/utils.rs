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

pub fn splitItemAndPrice(itemAndPrice: &str) -> Option<(&str, f32)> {
    // Split item and price using the dash as the separator
    let (itemPart, pricePart) = itemAndPrice.rsplit_once('-')?;

    // Trim leading and trailing whitespaces and the currency symbol (£)
    let price = pricePart.trim().trim_start_matches('£').trim().parse::<f32>().ok()?;

    // Return a tuple of both item and price
    return Some((itemPart.trim(), price));
}
#[test]
fn testSplitItemAndPrice() {
    assert_eq!(splitItemAndPrice("Keyboard - £15"), Some(("Keyboard", 15.0)));
    assert_eq!(splitItemAndPrice(" Mouse-And-Keyboard   -  £     45"), Some(("Mouse-And-Keyboard", 45.0)));
    assert_eq!(splitItemAndPrice(" Headset-£ 20"), Some(("Headset", 20.0)));
    assert_eq!(splitItemAndPrice("Invalid Line"), None);
}
