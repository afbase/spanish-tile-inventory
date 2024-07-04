# Spanish Tile Inventory Analysis

This project is a web application built with Rust and Yew for analyzing and visualizing Spanish ceramic tile inventory data.

## Features

- Interactive map displaying tile locations
- Inventory analysis with statistics
- Photo viewer for selected tiles
- Synchronized selection between map and analysis display

## Project Structure

- `app`: Main application logic
- `components`: Reusable UI components
- `data`: Data structures and analysis functions
- `utils`: Utility functions (e.g., CSV parsing)
- `web`: Web entry point and HTML template

## Setup

1. Install Rust and wasm-pack
2. Clone this repository
3. Run `wasm-pack build web --target web` in the project root
4. Serve the `web` directory using a local server

## Usage

Open the application in a web browser and interact with the map and analysis display to explore the Spanish tile inventory data.