use crate::map::assets::MAIN_TILEMAP;
use crate::map::chunks::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub const MAP_SIZE: i32 = 30;
pub const TILE_SIZE: f32 = 16.0;
pub const NOISE_SCALE: f64 = 0.12;
pub const DECOR_NOISE_SCALE: f64 = 0.99;
pub const GRID_X: u32 = 25;
pub const GRID_Y: u32 = 18;

fn get_tile_type(x: i32, y: i32, perlin: &Perlin) -> (bool, bool) {
    let val = (perlin.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]) + 1.0) / 2.0;
    let is_grass = val > 0.3 && val < 0.7;
    let is_water = val < 0.3;
    (is_grass, is_water)
}

pub fn spawn_chunk(
    commands: &mut Commands,
    cx: i32,
    cy: i32,
    perlin: &Perlin,
    layout: &Handle<TextureAtlasLayout>,
    tex: Handle<Image>,
    cols: u32,
    decor_layout: &Handle<TextureAtlasLayout>,
    decor_tex: Handle<Image>,
) {
    let start_x = cx * CHUNK_SIZE;
    let start_y = cy * CHUNK_SIZE;

    for y in start_y..(start_y + CHUNK_SIZE) {
        for x in start_x..(start_x + CHUNK_SIZE) {
            // --- 1. BASE TERRAIN ---
            let dirt_name = if rand::random::<f32>() > 0.95 {
                "dirt_2"
            } else {
                "dirt_1"
            };
            spawn_sprite(commands, dirt_name, x, y, 0.0, layout, tex.clone(), cols);

            let (is_grass, is_water) = get_tile_type(x, y, perlin);

            // WATER
            if is_water {
                let mask = calculate_mask_water(x, y, perlin);
                let has_grass_neighbor = check_grass_neighbor(x, y, perlin);
                let prefix = if has_grass_neighbor {
                    "water_grass"
                } else {
                    "water_dirt"
                };
                if let Some(s) = match_mask_to_suffix(mask) {
                    let name = if s == "mc" {
                        "water_mc".into()
                    } else {
                        format!("{}_{}", prefix, s)
                    };
                    spawn_sprite(commands, &name, x, y, 1.0, layout, tex.clone(), cols);
                }
            }

            // GRASS
            if is_grass {
                let mask = calculate_mask_grass(x, y, perlin);
                if let Some(s) = match_mask_to_suffix(mask) {
                    spawn_sprite(
                        commands,
                        &format!("grass_{}", s),
                        x,
                        y,
                        2.0,
                        layout,
                        tex.clone(),
                        cols,
                    );
                }

                // --- 2. DECORATION ---
                let get_val = |tx, ty| {
                    (perlin.get([
                        tx as f64 * DECOR_NOISE_SCALE,
                        ty as f64 * DECOR_NOISE_SCALE,
                        3.0,
                    ]) + 1.0)
                        / 2.0
                };
                let val = get_val(x, y);

                let local_x = x - start_x;
                let local_y = y - start_y;

                let near_edge = local_x < 3
                    || local_x > CHUNK_SIZE - 3
                    || local_y < 1
                    || local_y > CHUNK_SIZE - 6;

                if !near_edge {
                    // 1. DENSE FOREST
                    if val > 0.65 {
                        if x % 6 == 0 && y % 6 == 0 {
                            let tree_type = if val > 0.80 { "oak_tree" } else { "pine_tree" };
                            spawn_bundle(
                                commands,
                                tree_type,
                                x,
                                y,
                                decor_layout,
                                decor_tex.clone(),
                            );
                        }
                    }
                    // 2. LARGE OBSTACLES
                    else if val > 0.50 {
                        if x % 5 == 1 && y % 5 == 1 {
                            let obstacle = if val > 0.58 { "boulder" } else { "hollow_log" };
                            spawn_bundle(commands, obstacle, x, y, decor_layout, decor_tex.clone());
                        }
                    }
                    // 3. Small Decors
                    else if val > 0.35 {
                        if x % 3 == 0 && y % 3 == 0 {
                            let clutter = match (x + y) % 3 {
                                0 => "mushrooms",
                                1 => "large_bush",
                                _ => "weeds",
                            };
                            spawn_bundle(commands, clutter, x, y, decor_layout, decor_tex.clone());
                        }
                    }
                }
            }
        }
    }
}

// Helper Function
fn calculate_mask_water(x: i32, y: i32, perlin: &Perlin) -> i32 {
    let mut mask = 0;
    if get_tile_type(x, y + 1, perlin).1 {
        mask += 1;
    }
    if get_tile_type(x + 1, y, perlin).1 {
        mask += 2;
    }
    if get_tile_type(x, y - 1, perlin).1 {
        mask += 4;
    }
    if get_tile_type(x - 1, y, perlin).1 {
        mask += 8;
    }
    mask
}

fn calculate_mask_grass(x: i32, y: i32, perlin: &Perlin) -> i32 {
    let mut mask = 0;
    if get_tile_type(x, y + 1, perlin).0 {
        mask += 1;
    }
    if get_tile_type(x + 1, y, perlin).0 {
        mask += 2;
    }
    if get_tile_type(x, y - 1, perlin).0 {
        mask += 4;
    }
    if get_tile_type(x - 1, y, perlin).0 {
        mask += 8;
    }
    mask
}

fn check_grass_neighbor(x: i32, y: i32, perlin: &Perlin) -> bool {
    get_tile_type(x, y + 1, perlin).0
        || get_tile_type(x + 1, y, perlin).0
        || get_tile_type(x, y - 1, perlin).0
        || get_tile_type(x - 1, y, perlin).0
}

fn match_mask_to_suffix(mask: i32) -> Option<&'static str> {
    match mask {
        6 => Some("tl"),
        14 => Some("tc"),
        12 => Some("tr"),
        7 => Some("ml"),
        15 => Some("mc"),
        13 => Some("mr"),
        3 => Some("bl"),
        11 => Some("bc"),
        9 => Some("br"),
        _ => None,
    }
}

fn spawn_sprite(
    commands: &mut Commands,
    name: &str,
    x: i32,
    y: i32,
    z: f32,
    layout: &Handle<TextureAtlasLayout>,
    tex: Handle<Image>,
    cols: u32,
) {
    if let Some(sprite_def) = MAIN_TILEMAP.sprites.iter().find(|s| s.name == name) {
        let index = (sprite_def.pixel_y / 16 * cols) + (sprite_def.pixel_x / 16);
        commands.spawn((
            Sprite {
                image: tex,
                texture_atlas: Some(TextureAtlas {
                    layout: layout.clone(),
                    index: index as usize,
                }),
                ..default()
            },
            Transform::from_xyz(x as f32 * 16.0, y as f32 * 16.0, z),
        ));
    }
}

pub fn spawn_bundle(
    commands: &mut Commands,
    bundle_type: &str,
    base_x: i32,
    base_y: i32,
    layout: &Handle<TextureAtlasLayout>,
    tex: Handle<Image>,
) {
    match bundle_type {
        // --- TREES ---
        "oak_tree" | "oak_tree_bare" => {
            let prefix = if bundle_type == "oak_tree" {
                "oak_g"
            } else {
                "oak_d"
            };
            for row in 0..6 {
                for col in 0..5 {
                    let name = format!("{}_{}_{}", prefix, row, col);
                    spawn_sprite_decor(
                        commands,
                        &name,
                        base_x + col - 2,
                        base_y + (5 - row),
                        3.0,
                        layout,
                        tex.clone(),
                    );
                }
            }
        }
        "pine_tree" | "pine_tree_bare" => {
            let prefix = if bundle_type == "pine_tree" {
                "pine_g"
            } else {
                "pine_b"
            };
            for row in 0..5 {
                for col in 0..3 {
                    let name = format!("{}_{}_{}", prefix, row, col);
                    spawn_sprite_decor(
                        commands,
                        &name,
                        base_x + col - 1,
                        base_y + (4 - row),
                        3.0,
                        layout,
                        tex.clone(),
                    );
                }
            }
        }

        // --- LOGS ---
        "hollow_log" | "hollow_log_dirt" => {
            let prefix = if bundle_type == "hollow_log" {
                "log_g"
            } else {
                "log_d"
            };
            for row in 0..2 {
                for col in 0..3 {
                    let name = format!("{}_{}_{}", prefix, row, col);
                    spawn_sprite_decor(
                        commands,
                        &name,
                        base_x + col,
                        base_y - (1 - row),
                        2.5,
                        layout,
                        tex.clone(),
                    );
                }
            }
        }

        // --- STANDARD DECOR ---
        "large_bush" | "large_bush_dirt" | "small_bush" | "small_bush_dirt" | "mushrooms"
        | "mushrooms_dirt" | "weeds" | "reeds" | "rock" | "boulder" => {
            let (prefix, z_order) = match bundle_type {
                "large_bush" => ("bush_lg_g", 2.5),
                "large_bush_dirt" => ("bush_lg_d", 2.5),
                "small_bush" => ("bush_sm_g", 2.2),
                "small_bush_dirt" => ("bush_sm_d", 2.2),
                "mushrooms" => ("mush_g", 2.1),
                "mushrooms_dirt" => ("mush_d", 2.1),
                "weeds" => ("weed_g", 2.1),
                "reeds" => ("reed_g", 2.1),
                "rock" => ("rock_g1", 2.4),
                "boulder" => ("boulder_g1", 2.6),
                _ => ("bush_lg_g", 2.0),
            };

            for row in 0..2 {
                for col in 0..2 {
                    let name = format!("{}_{}_{}", prefix, row, col);
                    spawn_sprite_decor(
                        commands,
                        &name,
                        base_x + col,
                        base_y - (1 - row),
                        z_order,
                        layout,
                        tex.clone(),
                    );
                }
            }
        }
        _ => {}
    }
}

fn spawn_sprite_decor(
    commands: &mut Commands,
    name: &str,
    x: i32,
    y: i32,
    z_base: f32,
    layout: &Handle<TextureAtlasLayout>,
    tex: Handle<Image>,
) {
    use crate::map::assets::DECOR_TILEMAP;

    if let Some(index) = DECOR_TILEMAP.sprite_index(name) {
        let dynamic_z = z_base - (y as f32 * 0.001);

        commands.spawn((
            Sprite {
                image: tex,
                texture_atlas: Some(TextureAtlas {
                    layout: layout.clone(),
                    index,
                }),
                ..default()
            },
            Transform::from_xyz(x as f32 * 16.0, y as f32 * 16.0, dynamic_z),
        ));
    }
}
