// geosearch.js
let geosearchProvider;
let geosearch;

export function initGeoSearch(map) {
    geosearchProvider = new window.GeoSearch.OpenStreetMapProvider();
    geosearch = new window.GeoSearch.GeoSearchControl({
        provider: geosearchProvider,
        style: 'bar',
        showMarker: false,
        autoClose: true,
    });
    map.addControl(geosearch);
}

export async function geocodeAddress(address) {
    if (!geosearchProvider) {
        throw new Error("GeoSearch not initialized");
    }
    const results = await geosearchProvider.search({ query: address });
    if (results.length > 0) {
        return { lat: results[0].y, lng: results[0].x };
    }
    return null;
}