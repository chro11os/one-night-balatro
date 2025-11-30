use raylib::prelude::*;
use std::collections::HashMap;

pub struct GameAssets {
    pub tex_cards: HashMap<String, Texture2D>,
    pub tex_banner: Texture2D,
    pub tex_btn_play: Texture2D,
    pub tex_btn_discard: Texture2D,
    pub tex_btn_plus_active: Texture2D,
    pub tex_btn_plus_disabled: Texture2D,
    // NEW: Blue Panel for Enemy Info
    pub tex_panel_blue: Texture2D,
}

impl GameAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut tex_cards = HashMap::new();
        let suits = ["clubs", "diamonds", "hearts", "spades"];
        let ranks = ["02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K", "A"];

        for suit in suits.iter() {
            for rank in ranks.iter() {
                let key = format!("card_{}_{}", suit, rank);
                let path = format!("assets/Cards/{}.png", key);
                if let Ok(texture) = rl.load_texture(thread, &path) {
                    tex_cards.insert(key, texture);
                }
            }
        }

        if let Ok(back) = rl.load_texture(thread, "assets/Cards/card_back.png") {
            tex_cards.insert("card_back".to_string(), back);
        }

        let banner_path = "assets/ui/UI_Flat_Banner04a.png";
        let tex_banner = rl.load_texture(thread, banner_path)
            .expect("Failed to load UI banner");

        let play_path = "assets/ui/UI_Flat_Bar10a.png";
        let tex_btn_play = rl.load_texture(thread, play_path)
            .expect("Failed to load Play button");

        let disc_path = "assets/ui/UI_Flat_Bar07a.png";
        let tex_btn_discard = rl.load_texture(thread, disc_path)
            .expect("Failed to load Discard button");

        let plus_act_path = "assets/ui/UI_Flat_FrameSlot03a.png";
        let tex_btn_plus_active = rl.load_texture(thread, plus_act_path)
            .expect("Failed to load Plus Active button");

        let plus_dis_path = "assets/ui/UI_Flat_FrameSlot01c.png";
        let tex_btn_plus_disabled = rl.load_texture(thread, plus_dis_path)
            .expect("Failed to load Plus Disabled button");

        // NEW: Load Blue Panel
        let panel_path = "assets/ui/UI_Flat_Frame02a.png";
        let tex_panel_blue = rl.load_texture(thread, panel_path)
            .expect("Failed to load Blue Panel");

        Self {
            tex_cards,
            tex_banner,
            tex_btn_play,
            tex_btn_discard,
            tex_btn_plus_active,
            tex_btn_plus_disabled,
            tex_panel_blue,
        }
    }
}