use raylib::prelude::*;

pub struct GameAssets {
    pub tex_spritesheet: Texture2D,
    pub tex_background: Texture2D,
    pub tex_banner: Texture2D,
    pub tex_btn_play: Texture2D,
    pub tex_btn_discard: Texture2D,
    pub tex_btn_plus_active: Texture2D,
    pub tex_btn_plus_disabled: Texture2D,
    pub tex_panel_blue: Texture2D,
    pub tex_panel_orange: Texture2D,
}

impl GameAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut tex_spritesheet = rl.load_texture(thread, "assets/resprite-cards/spritesheet.png")
            .expect("Failed to load spritesheet at assets/resprite-cards/spritesheet.png");

        tex_spritesheet.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

        // FIX: Try to load the new background; fall back to the old one if missing to prevent crash.
        let tex_background = rl.load_texture(thread, "assets/bg/Final_bg.png")
            .or_else(|_| {
                println!("WARNING: Could not find 'assets/Final_bg.png'. Reverting to default background.");
                rl.load_texture(thread, "assets/bg/Mini_Text_Castle.png")
            })
            .expect("Failed to load background (checked both Final_bg.png and Mini_Text_Castle.png)");

        let banner_path = "assets/ui/UI_Flat_Banner04a.png";
        let tex_banner = rl.load_texture(thread, banner_path).expect("Failed to load UI banner");

        let play_path = "assets/ui/UI_Flat_Bar10a.png";
        let tex_btn_play = rl.load_texture(thread, play_path).expect("Failed to load Play button");

        let disc_path = "assets/ui/UI_Flat_Bar07a.png";
        let tex_btn_discard = rl.load_texture(thread, disc_path).expect("Failed to load Discard button");

        let plus_act_path = "assets/ui/UI_Flat_FrameSlot03a.png";
        let tex_btn_plus_active = rl.load_texture(thread, plus_act_path).expect("Failed to load Plus Active");

        let plus_dis_path = "assets/ui/UI_Flat_FrameSlot01c.png";
        let tex_btn_plus_disabled = rl.load_texture(thread, plus_dis_path).expect("Failed to load Plus Disabled");

        let panel_blue_path = "assets/ui/UI_Flat_Frame02a.png";
        let tex_panel_blue = rl.load_texture(thread, panel_blue_path).expect("Failed to load Blue Panel");

        let panel_orange_path = "assets/ui/UI_Flat_Frame03a.png";
        let tex_panel_orange = rl.load_texture(thread, panel_orange_path).expect("Failed to load Orange Panel");

        Self {
            tex_spritesheet,
            tex_background,
            tex_banner,
            tex_btn_play,
            tex_btn_discard,
            tex_btn_plus_active,
            tex_btn_plus_disabled,
            tex_panel_blue,
            tex_panel_orange,
        }
    }
}