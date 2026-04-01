# mca-rust
A Rust tool that allows you to verify in a simple man# Minecraft Region to Setblock Exporter

A Rust command-line tool that reads Minecraft `.mca` region files and outputs a `setblocks.txt` file containing `/setblock` commands for every block in the world — ready to use in data pack functions or command blocks.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Minecraft Java Edition region files (`.mca` format)

## Setup

1. Clone the repository.
2. Open `src/main.rs` and set `caminho_folder` to the path of your Minecraft world's `region` folder:

   ```rust
   let caminho_folder = r"C:\Users\you\AppData\Roaming\.minecraft\saves\MyWorld\region\";
   ```

3. Optionally set `detetar_ar = true` if you want air blocks included in the output (off by default).

4. Build and run:

   ```bash
   cargo run --release
   ```

The output file `setblocks.txt` will be created in the directory where you run the command.

## Output format

```
setblock -512 64 -512 minecraft:stone
setblock -511 64 -512 minecraft:grass_block
...
```

Each line is a standard Minecraft setblock command, usable in:
- `.mcfunction` files (data packs)
- Command blocks
- World editors that accept command lists

## Notes

- Scans region files in the range `r.-32.-32.mca` to `r.31.31.mca` (a ~16,000×16,000 block area). Missing region files are skipped automatically.
- For large worlds the output file can be very large — consider narrowing the loop range in the source if needed.
- Block states (e.g. door facing, slab type) are not exported, only the block name.

## Dependencies

- [`fastnbt`](https://crates.io/crates/fastnbt) — NBT data parsing
- [`mca-parser`](https://crates.io/crates/mca-parser) — `.mca` region file parsingner what's in each block in a Minecraft region (by default, exports as "setblock material posx posy posz " format.
