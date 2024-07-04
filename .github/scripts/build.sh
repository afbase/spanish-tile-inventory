#!/bin/bash
set -eux pipefail

# Environment variables
GITHUB_TOKEN=${GITHUB_TOKEN:-}
SPANISH_TILES_REPO=${SPANISH_TILES_REPO:-"spanish-tile-inventory"}
WEBSITE_REPO=${WEBSITE_REPO:-"afbase.github.io"}
BLOG_FOLDER="${WEBSITE_REPO}/content/blog/spanish-tiles-inventory"
YEW_APP="web"
RUST_CHANNEL=${RUST_CHANNEL:-"nightly"}
RUST_DATE=${RUST_DATE:-"2023-07-01"}
RUST_HOST=${RUST_HOST:-"x86_64-unknown-linux-gnu"}
RUST_TOOLCHAIN="${RUST_CHANNEL}-${RUST_DATE}-${RUST_HOST}"
TRUNK_VERSION=${TRUNK_VERSION:-"0.17.5"}

init_variables() {
    WORK_DIR=$(pwd)
    SPANISH_TILES_REPO_PATH="${WORK_DIR}/${SPANISH_TILES_REPO}"
    WEBSITE_REPO_PATH="${WORK_DIR}/${WEBSITE_REPO}"
}

set_rust_toolchain() {
    local toolchain="$1"
    echo "Setting Rust toolchain to: $toolchain"
    
    if ! rustup toolchain list | grep -q "$toolchain"; then
        echo "Toolchain $toolchain is not installed. Installing now..."
        rustup toolchain install "$toolchain" || {
            echo "Failed to install toolchain $toolchain"
            return 1
        }
    fi
    
    rustup default "$toolchain" || {
        echo "Failed to set toolchain $toolchain as default"
        return 1
    }
    
    current_toolchain=$(rustup show active-toolchain)
    echo "Current active toolchain: $current_toolchain"
    
    rustup target add wasm32-unknown-unknown --toolchain "$toolchain" || {
        echo "Failed to add wasm32-unknown-unknown target for toolchain $toolchain"
        return 1
    }
    
    echo "Rust toolchain setup completed successfully"
}

clone_repositories() {
    git clone --depth 1 "https://${GITHUB_TOKEN}@github.com/afbase/${SPANISH_TILES_REPO}"
    git clone --depth 1 "https://${GITHUB_TOKEN}@github.com/afbase/${WEBSITE_REPO}"
}

clean_old_wasm_files() {
    mapfile -t WASM < <(ls -1 "${BLOG_FOLDER}" | grep "${YEW_APP}" | grep -v index.html | grep -v old | grep -v png | grep -v jpg | grep -v md | grep -v webp | tr '\n' ' ')
    
    echo "Files for ${YEW_APP} in ${BLOG_FOLDER}:"
    if [ ${#WASM[@]} -eq 0 ]; then
        echo "  No files found"
    else
        printf '  %s\n' "${WASM[@]}"
    fi
    
    for wasm_file in "${WASM[@]}"; do
        file_path="${BLOG_FOLDER}/${wasm_file}"
        if [ -f "$file_path" ]; then
            rm -v "$file_path"
        else
            echo "File not found: $file_path"
        fi
    done
}

build_wasm() {
    repo="${SPANISH_TILES_REPO_PATH}/${YEW_APP}"
    target="${repo}/index.html"
    
    echo "Building WASM for ${YEW_APP}"
    echo "Memory Usage:"
    free -m | awk 'NR==2{printf "%s/%sMB (%.2f%%)\n", $3,$2,$3*100/$2 }'
    echo "Disk Usage:"
    df -h | awk '$NF=="/"{printf "%d/%dGB (%s)\n", $3,$2,$5}'
    echo "CPU Load:"
    top -bn1 | grep load | awk '{printf "%.2f\n", $(NF-2)}'
    
    cd "${repo}" && \
    rustup run "${RUST_TOOLCHAIN}" trunk build --release "${target}"
    
    if [ $? -ne 0 ]; then
        echo "Failed to build WASM for ${YEW_APP}"
        return 1
    fi
}

process_js_wasm_files() {
    app_template_path="${WEBSITE_REPO_PATH}/templates/shortcodes/${YEW_APP}.html"
    repo="${SPANISH_TILES_REPO_PATH}/${YEW_APP}"
    dist="${repo}/dist"
    
    readarray -t WASM < <(ls -1 "${dist}" | grep -v index.html | grep -v old)
    
    echo "Files for ${YEW_APP} in ${dist}:"
    if [ ${#WASM[@]} -eq 0 ]; then
        echo "  No files found"
    else
        printf '  %s\n' "${WASM[@]}"
    fi
    
    for wasm_file in "${WASM[@]}"; do
        if [[ $wasm_file =~ ^(.+)\.([^.]+)$ ]]; then
            file_name="${BASH_REMATCH[1]}"
            file_ext="${BASH_REMATCH[2]}"
        else
            echo "Error: Unable to parse filename and extension for $wasm_file"
            continue
        fi
        
        echo "Processing file: $wasm_file (name: $file_name, extension: $file_ext)"
        
        if [ "${file_ext}" = "js" ]; then
            wasm_blob="${file_name}_bg.wasm"
            echo "<div id=\"${YEW_APP}\">" > "${app_template_path}"
            {
                echo '<script type="module">'
                echo "    import init from './${wasm_file}';"
                echo "    init('./${wasm_blob}');"
                echo "</script>"
                echo "</div>"
            } >> "${app_template_path}"
        fi
        
        if [ "${file_ext}" = "wasm" ]; then
            wasm_blob_old="${file_name}.wasm.old"
            mv "${dist}/${wasm_file}" "${dist}/${wasm_blob_old}"
            wasm-opt -Oz -o "${dist}/${wasm_file}" "${dist}/${wasm_blob_old}"
        fi
        
        mkdir -p "${BLOG_FOLDER}"
        if [ -f "${dist}/${wasm_file}" ]; then
            cp "${dist}/${wasm_file}" "${BLOG_FOLDER}/${wasm_file}"
        else
            echo "Warning: File not found: ${dist}/${wasm_file}"
        fi
    done
}

main() {
    init_variables
    set_rust_toolchain "$RUST_TOOLCHAIN" || {
        echo "Failed to set Rust toolchain"
        exit 1
    }
    clone_repositories
    clean_old_wasm_files
    build_wasm
    process_js_wasm_files
}

main