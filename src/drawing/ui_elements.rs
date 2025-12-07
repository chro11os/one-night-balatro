use raylib::prelude::*;

pub fn get_button_offset(d: &RaylibHandle, rect: Rectangle) -> (f32, f32) {
    let mouse_pos = d.get_mouse_position();
    let is_hovered = rect.check_collision_point_rec(mouse_pos);
    let is_down = d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

    if is_hovered {
        if is_down { return (4.0, 0.0); }
        else { return (-2.0, 6.0); }
    }
    (0.0, 3.0)
}
