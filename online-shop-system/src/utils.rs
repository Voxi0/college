use ratatui::{
    Frame,
    layout::{Layout, Rect, Constraint, Flex},
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

pub fn splitItemAndPrice(itemAndPrice: &str) -> Option<(&str, u32)> {
    // Split item and price using the dash as the separator
    let (itemPart, pricePart) = itemAndPrice.rsplit_once('-')?;

    // Trim leading and trailing whitespaces and the currency symbol (£)
    let price = pricePart.trim().trim_start_matches('£').trim().parse::<u32>().ok()?;

    // Return a tuple of both item and price
    return Some((itemPart.trim(), price));
}
#[test]
fn testSplitItemAndPrice() {
    assert_eq!(splitItemAndPrice("Keyboard - £15"), Some(("Keyboard", 15)));
    assert_eq!(splitItemAndPrice(" Mouse-And-Keyboard   -  £     45"), Some(("Mouse-And-Keyboard", 45)));
    assert_eq!(splitItemAndPrice(" Headset-£ 20"), Some(("Headset", 20)));
    assert_eq!(splitItemAndPrice("Invalid Line"), None);
}
