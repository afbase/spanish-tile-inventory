use yew::prelude::*;
use web_sys::HtmlSelectElement;
use std::collections::HashSet;
use std::path::PathBuf;
use data::inventory::TileInventory;
use data::analysis::{analyze_inventory, InventoryAnalysis};

/// Represents the state and logic for the Analysis Display component
pub struct AnalysisDisplay {
    analysis: InventoryAnalysis,
    selected_street: Option<String>,
    selected_address: Option<String>,
    current_photo_index: usize,
}

/// Enumeration of possible messages that can update the AnalysisDisplay component
pub enum Msg {
    /// Triggered when a street is selected from the dropdown
    /// Input: String - The name of the selected street
    StreetSelected(String),
    /// Triggered when an address is selected from the dropdown
    /// Input: String - The selected address
    AddressSelected(String),
    /// Triggered when the user wants to view the next photo
    NextPhoto,
    /// Triggered when the user wants to view the previous photo
    PreviousPhoto,
}

/// Properties for the AnalysisDisplay component
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The complete inventory of tile items
    pub inventory: Vec<TileInventory>,
    /// Callback function to notify parent component of selection changes
    /// Input: Option<TileInventory> - The currently selected inventory item, if any
    pub on_selection: Callback<Option<TileInventory>>,
}

impl Component for AnalysisDisplay {
    type Message = Msg;
    type Properties = Props;

    /// Creates a new instance of the AnalysisDisplay component
    /// Input: 
    ///   - ctx: &Context<Self> - The component's context
    /// Output: Self - A new instance of AnalysisDisplay
    fn create(ctx: &Context<Self>) -> Self {
        let analysis = analyze_inventory(&ctx.props().inventory);
        Self {
            analysis,
            selected_street: None,
            selected_address: None,
            current_photo_index: 0,
        }
    }

    /// Handles updates to the component's state based on received messages
    /// Inputs:
    ///   - ctx: &Context<Self> - The component's context
    ///   - msg: Self::Message - The message triggering the update
    /// Output: bool - Whether the component should re-render
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StreetSelected(street) => {
                self.selected_street = Some(street);
                self.selected_address = None;
                self.current_photo_index = 0;
                self.notify_selection(ctx);
                true
            }
            Msg::AddressSelected(address) => {
                self.selected_address = Some(address);
                self.current_photo_index = 0;
                self.notify_selection(ctx);
                true
            }
            Msg::NextPhoto => {
                if let Some(max) = self.get_photo_count(ctx) {
                    self.current_photo_index = (self.current_photo_index + 1) % max;
                }
                true
            }
            Msg::PreviousPhoto => {
                if let Some(max) = self.get_photo_count(ctx) {
                    self.current_photo_index = (self.current_photo_index + max - 1) % max;
                }
                true
            }
        }
    }

    /// Renders the component's view
    /// Input: 
    ///   - ctx: &Context<Self> - The component's context
    /// Output: Html - The rendered view
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Collect unique street signs
        let street_signs: Vec<String> = ctx.props().inventory.iter()
            .map(|item| item.street_sign.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // Filter addresses based on selected street
        let addresses: Vec<String> = if let Some(street) = &self.selected_street {
            ctx.props().inventory.iter()
                .filter(|item| &item.street_sign == street)
                .map(|item| item.street_address.clone())
                .collect()
        } else {
            vec![]
        };

        html! {
            <div class="card mb-4">
                <div class="card-body">
                    <h2 class="card-title">{"Inventory Analysis"}</h2>
                    <p class="card-text">{format!("Total Items: {}", self.analysis.total_items)}</p>
                    <p class="card-text">{format!("Total Damaged Tiles: {}", self.analysis.total_damaged_tiles)}</p>
                    <p class="card-text">{format!("Average Damaged Tiles: {:.2}", self.analysis.average_damaged_tiles)}</p>

                    <h3>{"Photo Album"}</h3>
                    <div class="mb-3">
                        <select class="form-select" onchange={ctx.link().callback(|e: Event| Msg::StreetSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                            <option selected=true disabled=true>{"Select Street Sign"}</option>
                            { for street_signs.iter().map(|street| html! { <option value={street.clone()}>{street}</option> }) }
                        </select>
                    </div>
                    <div class="mb-3">
                        <select class="form-select" disabled={self.selected_street.is_none()} onchange={ctx.link().callback(|e: Event| Msg::AddressSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                            <option selected=true disabled=true>{"Select Address"}</option>
                            { for addresses.iter().map(|address| html! { <option value={address.clone()}>{address}</option> }) }
                        </select>
                    </div>
                    { self.render_photo_viewer(ctx) }
                </div>
            </div>
        }
    }
}

impl AnalysisDisplay {
    /// Renders the photo viewer component
    /// Input:
    ///   - ctx: &Context<Self> - The component's context
    /// Output: Html - The rendered photo viewer
    fn render_photo_viewer(&self, ctx: &Context<Self>) -> Html {
        if let Some(item) = self.get_selected_item(&ctx.props().inventory) {
            let photos = self.get_photos(item);
            if !photos.is_empty() {
                let photo_src = photos[self.current_photo_index].to_str().unwrap_or("").to_string();
                html! {
                    <div class="photo-viewer">
                        <img src={photo_src} alt="Ceramic Sign" class="img-fluid" />
                        <div class="mt-2">
                            <button class="btn btn-secondary me-2" onclick={ctx.link().callback(|_| Msg::PreviousPhoto)}>{"Previous"}</button>
                            <button class="btn btn-secondary" onclick={ctx.link().callback(|_| Msg::NextPhoto)}>{"Next"}</button>
                        </div>
                        <p>{format!("Photo {} of {}", self.current_photo_index + 1, photos.len())}</p>
                    </div>
                }
            } else {
                html! { <p>{"No photos available for this item."}</p> }
            }
        } else {
            html! {}
        }
    }

    /// Retrieves the currently selected inventory item
    /// Inputs:
    ///   - inventory: &[TileInventory] - The full inventory to search
    /// Output: Option<&TileInventory> - The selected item, if any
    fn get_selected_item<'a>(&self, inventory: &'a [TileInventory]) -> Option<&'a TileInventory> {
        inventory.iter().find(|item| {
            Some(&item.street_sign) == self.selected_street.as_ref() &&
            Some(&item.street_address) == self.selected_address.as_ref()
        })
    }

    /// Retrieves all available photos for a given inventory item
    /// Input:
    ///   - item: &TileInventory - The inventory item to get photos for
    /// Output: Vec<PathBuf> - A vector of paths to the item's photos
    fn get_photos(&self, item: &TileInventory) -> Vec<PathBuf> {
        vec![
            item.photo_1.clone(),
            item.photo_2.clone(),
            item.photo_3.clone(),
            item.photo_4.clone(),
            item.photo_5.clone(),
        ].into_iter().flatten().collect()
    }

    /// Retrieves the number of photos for the currently selected item
    /// Input:
    ///   - ctx: &Context<Self> - The component's context
    /// Output: Option<usize> - The number of photos, if an item is selected
    fn get_photo_count(&self, ctx: &Context<Self>) -> Option<usize> {
        self.get_selected_item(&ctx.props().inventory)
            .map(|item| self.get_photos(item).len())
    }

    /// Notifies the parent component of a selection change
    /// Input:
    ///   - ctx: &Context<Self> - The component's context
    fn notify_selection(&self, ctx: &Context<Self>) {
        let selected_item = self.get_selected_item(&ctx.props().inventory).cloned();
        ctx.props().on_selection.emit(selected_item);
    }
}