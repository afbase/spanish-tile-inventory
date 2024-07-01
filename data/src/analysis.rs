use crate::inventory::TileInventory;

pub struct InventoryAnalysis {
    pub total_items: usize,
    pub total_damaged_tiles: u32,
    pub average_damaged_tiles: f64,
}

pub fn analyze_inventory(inventory: &[TileInventory]) -> InventoryAnalysis {
    let total_items = inventory.len();
    let total_damaged_tiles: u32 = inventory.iter().map(|item| item.number_of_tiles_damaged).sum();
    let average_damaged_tiles = total_damaged_tiles as f64 / total_items as f64;

    InventoryAnalysis {
        total_items,
        total_damaged_tiles,
        average_damaged_tiles,
    }
}