use raylib::prelude::*;
use std::collections::HashMap;

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

    // Store Rune Icons
    pub rune_icons: HashMap<String, Texture2D>,
}

impl GameAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // 1. Load Standard Assets
        let mut tex_spritesheet = rl.load_texture(thread, "assets/resprite-cards/spritesheet.png")
            .expect("Failed to load spritesheet.");
        tex_spritesheet.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

        let tex_background = rl.load_texture(thread, "assets/Final_bg.png")
            .or_else(|_| rl.load_texture(thread, "assets/bg/Final_bg.png"))
            .expect("Failed to load background.");

        let banner_path = "assets/ui/UI_Flat_Banner04a.png";
        let tex_banner = rl.load_texture(thread, banner_path).expect("Failed to load UI banner");

        let play_path = "assets/ui/UI_Flat_Bar10a.png";
        let tex_btn_play = rl.load_texture(thread, play_path).expect("Failed to load Play button");

        let disc_path = "assets/ui/UI_Flat_Bar07a.png";
        let tex_btn_discard = rl.load_texture(thread, disc_path).expect("Failed to load Discard button");

        let tex_btn_plus_active = rl.load_texture(thread, play_path).unwrap();
        let tex_btn_plus_disabled = rl.load_texture(thread, disc_path).unwrap();
        let tex_panel_blue = rl.load_texture(thread, banner_path).unwrap();
        let tex_panel_orange = rl.load_texture(thread, banner_path).unwrap();

        // 2. Load Rune Icons
        let mut rune_icons = HashMap::new();

        let categories = [
            ("combat_style", vec!["Paladin", "Reaper", "Judgement"]),
            ("economy", vec!["Investment", "Mentalist", "Merchant"]),
            ("utility", vec!["Evolution", "Greed", "Midas"]),
            // ADDED: starting_bonus category
            ("starting_bonus", vec!["Force", "Flow", "Wealth"]),
        ];

        for (folder, names) in categories.iter() {
            for name in names {
                let path = format!("assets/rune_icons/{}/{}_icon.png", folder, name.to_lowercase());

                if let Ok(tex) = rl.load_texture(thread, &path) {
                    tex.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
                    rune_icons.insert(name.to_string(), tex);
                    println!("Loaded Icon: {}", name);
                } else {
                    println!("> Warning: Icon not found: {}", path);
                }
            }
        }

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
            rune_icons,
        }
    }
}