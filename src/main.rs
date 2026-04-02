
use std::fs::File;
use std::io::Write;
use std::path::Path;
use mca_parser::Region;

fn get_palette_index(data: &[i64], block_index: usize, bits_per_block: usize) -> usize {
    let values_per_long = 64 / bits_per_block;
    let long_index = block_index / values_per_long;
    let index_in_long = block_index % values_per_long;
    let bit_offset = index_in_long * bits_per_block;
    let mask = (1u64 << bits_per_block) - 1;

    let long = data.get(long_index).copied().unwrap_or(0) as u64;
    ((long >> bit_offset) & mask) as usize
}

fn main() {
    let caminho_folder = r"";//Input the path to a regions file in a minecraft save
    let mut output = File::create("setblocks.txt").expect("Failed to create scheme.txt"); //This file will store: a block's id (in the format minecraft:material) and the position in posX posY posZ, using the syntaxe of minecraft setblock: setblock posX posY posZ minecraft:material  
    let detetar_ar = false;

    for primeiro_numero in -32..32 {
        for segundo_numero in -32..32 {
            let nome_ficheiro = format!("r.{}.{}.mca", primeiro_numero, segundo_numero);
            let caminho = format!("{}{}", caminho_folder, nome_ficheiro);
            let path = Path::new(&caminho);

            if !path.exists() {
                continue;
            }

            println!("{}", caminho);

            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let region = match Region::from_reader(&mut file) {
                Ok(r) => r,
                Err(_) => continue,
            };

            for chunk_x_local in 0..32u32 {
                for chunk_z_local in 0..32u32 {
                    if let Ok(Some(chunk)) = region.get_chunk(chunk_x_local, chunk_z_local) {
                        let parsed = match chunk.parse() {
                            Ok(p) => p,
                            Err(_) => continue,
                        };

                        let chunk_x = parsed.x_pos;
                        let chunk_z = parsed.z_pos;

                        for section in &parsed.sections {
                            let block_states = match &section.block_states {
                                Some(bs) => bs,
                                None => continue,
                            };

                            if block_states.palette.is_empty() {
                                continue;
                            }

                            let section_y = section.y as i32;

                            if let Some(data) = &block_states.data {
                                let palette_len = block_states.palette.len();
                                let bits_per_block = std::cmp::max(
                                    4,
                                    (usize::BITS - (palette_len.saturating_sub(1)).leading_zeros()) as usize,
                                );

                                for block_index in 0..4096 {
                                    let palette_index =
                                        get_palette_index(data, block_index, bits_per_block);

                                    let block_state = match block_states.palette.get(palette_index) {
                                        Some(b) => b,
                                        None => continue,
                                    };

                                    let material = format!("minecraft:{}", block_state.name.key);

                                    if !detetar_ar && material == "minecraft:air" {
                                        continue;
                                    }

                                    let local_x = (block_index & 15) as i32;
                                    let local_z = ((block_index >> 4) & 15) as i32;
                                    let local_y = ((block_index >> 8) & 15) as i32;

                                    let world_x = chunk_x * 16 + local_x;
                                    let world_y = section_y * 16 + local_y;
                                    let world_z = chunk_z * 16 + local_z;

                                    writeln!(output, "setblock {} {} {} {}", world_x, world_y, world_z, material)
                                        .expect("Failed to write block");
                                }
                            } else {
                                let block_state = &block_states.palette[0];
                                let material = format!("minecraft:{}", block_state.name.key);

                                if !detetar_ar && material == "minecraft:air" {
                                    continue;
                                }

                                for local_y in 0..16i32 {
                                    for local_z in 0..16i32 {
                                        for local_x in 0..16i32 {
                                            let world_x = chunk_x * 16 + local_x;
                                            let world_y = section_y * 16 + local_y;
                                            let world_z = chunk_z * 16 + local_z;

                                            writeln!(output, "setblock {} {} {} {}", world_x, world_y, world_z, material)
                                                .expect("Failed to write block");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}