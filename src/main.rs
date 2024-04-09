use enigo::*;

fn main() {
    let mut enigo = Enigo::new();

    // coords of tos search-bar
    const X: i32 = 595;
    const Y: i32 = 149;

    let (old_x, old_y) = enigo.mouse_location();

    // go to search-bar and click
    enigo.mouse_move_to(X, Y);
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_up(MouseButton::Left);

    enigo.key_down(Key::Control);
    enigo.key_down(Key::Layout('a'));
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Layout('a'));

    enigo.mouse_move_to(old_x, old_y);
}
