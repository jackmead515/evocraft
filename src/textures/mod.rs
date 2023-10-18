use std::collections::HashMap;

use macroquad::prelude::*;

async fn infer_texture(base_dir: &str, path: &str) -> Texture2D {
    return load_texture(&format!("{}{}{}", base_dir, path, ".png")).await.unwrap();
}

pub struct TextureMap {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureMap {

    pub async fn load() -> TextureMap {

        let mut textures = HashMap::new();

        let base_dir = "assets/dungeon/";

        let texture_list = vec![
            "player/base/human_male",
            "monster/eyes/giant_eyeball",
            "dungeon/floor/grass/grass0-dirt-mix_1",
            "dungeon/floor/grass/grass0-dirt-mix_2",
            "dungeon/floor/grass/grass0-dirt-mix_3",
            "dungeon/wall/brick_gray_0",
            "dungeon/wall/brick_gray_1",
            "dungeon/wall/brick_gray_2",
            "dungeon/wall/brick_gray_3",
            "dungeon/floor/dirt_0_new",
            "dungeon/floor/floor_sand_stone_0",
            "dungeon/floor/limestone_0",
            "dungeon/floor/mud_0",
            "misc/brands/top_right/sleeping"
        ];

        for text_id in texture_list {
            textures.insert(text_id.to_string(), infer_texture(base_dir, text_id).await);
        }

        return TextureMap {
            textures: textures,
        };
    }

    pub fn get(&self, texture_id: &str) -> &Texture2D {
        return self.textures.get(texture_id).unwrap();
    }

}


pub async fn load() -> TextureMap {
    let texture_map = TextureMap::load().await;

    build_textures_atlas();

    return texture_map;
}