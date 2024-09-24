use std::collections::HashMap;

use macroquad::prelude::*;

async fn infer_texture(base_dir: &str, path: &str) -> Texture2D {
    return load_texture(&format!("{}{}{}", base_dir, path, ".png")).await.unwrap();
}

pub struct TextureMap {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureMap {

    pub async fn new() -> TextureMap {

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
            

            // beach
            "dungeon/floor/sand_1",
            "dungeon/floor/sand_2",
            "dungeon/floor/sand_3",
            "dungeon/floor/sand_4",
            "dungeon/floor/sand_5",
            "dungeon/floor/sand_6",
            "dungeon/floor/sand_7",
            "dungeon/floor/sand_8",

            "dungeon/floor/dirt_0_new",

            // mountain
            "dungeon/floor/mud_0",
            "dungeon/floor/mud_1",
            "dungeon/floor/pebble_brown_0_new",
            "dungeon/floor/pebble_brown_1_new",
            "dungeon/floor/grey_dirt_0_new",
            "dungeon/floor/grey_dirt_1_new",

            "dungeon/floor/pebble_brown_2_new",
            "dungeon/floor/pebble_brown_3_new",
            "dungeon/floor/pebble_brown_4_new",
            "dungeon/floor/pebble_brown_5_new",
            "dungeon/floor/pebble_brown_6_new",
            "dungeon/floor/pebble_brown_7_new",
            "dungeon/floor/pebble_brown_8_new",

            "dungeon/floor/limestone_0",
            

            "dungeon/trees/tree_1_lightred",
            "dungeon/trees/tree_1_red",
            "dungeon/trees/tree_1_yellow",
            "dungeon/trees/tree_2_lightred",
            "dungeon/trees/tree_2_red",
            "dungeon/trees/tree_2_yellow",



            "misc/brands/top_right/sleeping",
            "dungeon/water/deep_water",
            "dungeon/water/deep_water_2",
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
    let texture_map = TextureMap::new().await;

    build_textures_atlas();

    return texture_map;
}