mod window_init;
mod draw_scene;

use raylib::prelude::*;
use crate::window_init::initialize_window;

fn main() 
{
    initialize_window();    
}
