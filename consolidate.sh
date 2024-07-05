#!/bin/bash

# This script consolidates a Rust git repository into a single file and can recreate the repository from this file.
# It captures .rs, .toml, .md, and .html files, as well as files referenced by include_bytes! macros.
# Binary files up to 200 KB are encoded in base64; larger binary files are ignored.

# Function to encode file content
# Inputs:
#   $1: File path
#   $2: Output file
# Output: Appends encoded content to the output file, or ignores if binary > 200 KB
encode_file_content() {
    local file=$1
    local output_file=$2
    local max_size=$((200 * 1024))  # 200 KB in bytes

    if [[ ! -f "$file" ]]; then
        echo "Warning: File not found: $file" >&2
        return
    fi

    local file_size
    file_size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null)
    
    if [[ ! "$file_size" =~ ^[0-9]+$ ]]; then
        echo "Warning: Could not determine file size for $file" >&2
        return
    fi

    if file -b --mime-type "$file" | grep -qE '^text/|^application/json'; then
        {
            echo "---"
            echo "file path: $file"
            echo ""
            cat "$file"
            echo ""
        } >> "$output_file"
    elif [[ $file_size -le $max_size ]]; then
        {
            echo "---"
            echo "file path: $file"
            echo ""
            echo "BASE64_ENCODED"
            base64 "$file"
            echo ""
        } >> "$output_file"
    else
        echo "Ignoring large binary file: $file ($(($file_size / 1024)) KB)" >&2
    fi
}

# Function for consolidating the repository
# Inputs:
#   $1: Repository path
#   $2: Output file name
# Output: Creates a consolidated file
consolidate() {
    local repo_path=$1
    local output_file=$2

    cd "$repo_path" || exit 1
    true > "$output_file"

    find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.html" \) \
        -not -path "*/target/*" \
        -not -path "*/.git/*" \
        -not -path "*/target" \
        -not -path "*/.git" \
        -print0 | while IFS= read -r -d '' file; do
        encode_file_content "$file" "$output_file"
        
        if [[ "$file" == *.rs ]]; then
            local current_file="$file"
            while IFS= read -r line; do
                if [[ "$line" =~ include_bytes!\(\"(.+)\"\) ]]; then
                    included_file="${BASH_REMATCH[1]}"
                    included_file_path="$(dirname "$current_file")/$included_file"
                    if [ -f "$included_file_path" ]; then
                        encode_file_content "$included_file_path" "$output_file"
                    fi
                fi
            done < "$current_file"
        fi
    done

    echo "Repository consolidated into $output_file"
}

# Function to recreate the repository from a consolidated file
# Inputs:
#   $1: Input file name
#   $2: Output directory
# Output: Recreates the repository structure
recreate() {
    local input_file=$1
    local output_dir=$2

    mkdir -p "$output_dir"
    cd "$output_dir" || exit 1

    local current_file=""
    local is_base64=false
    while IFS= read -r line; do
        if [[ $line == ---* ]]; then
            current_file=""
            is_base64=false
        elif [[ $line == file\ path:* ]]; then
            current_file=${line#file path: }
            mkdir -p "$(dirname "$current_file")"
            true > "$current_file"
        elif [[ $line == BASE64_ENCODED ]]; then
            is_base64=true
        elif [[ -n $current_file ]]; then
            if $is_base64; then
                echo "$line" | base64 -d >> "$current_file"
            else
                echo "$line" >> "$current_file"
            fi
        fi
    done < "$input_file"

    echo "Repository recreated in $output_dir"
}

# Function to display usage information
# Inputs: None
# Output: Prints usage information to stdout
usage() {
    cat << EOF
Rust Repository Consolidation and Recreation Tool

Usage:
  $0 consolidate <path_to_rust_repository> <output_file_name>
  $0 recreate <input_file_name> <directory_where_this_will_be_recreated>

Commands:
  consolidate  Consolidate a Rust repository into a single file
  recreate     Recreate a repository structure from a consolidated file

Arguments:
  path_to_rust_repository  The path to the root of the Rust git repository
  output_file_name         The name of the file to store the consolidated repository
  input_file_name          The name of the consolidated file to recreate from
  directory_where_this_will_be_recreated  The directory to recreate the repository in

Examples:
  $0 consolidate /path/to/your/rust/repo consolidated_repo.txt
  $0 recreate consolidated_repo.txt /path/to/recreate

Note: Binary files larger than 200 KB are ignored during consolidation.
EOF
}

# Main function to handle script logic
# Inputs: Command-line arguments
# Output: Executes the appropriate function based on the command
main() {
    if [ $# -lt 3 ]; then
        usage
        exit 1
    fi

    case $1 in
        consolidate)
            consolidate "$2" "$3"
            ;;
        recreate)
            recreate "$2" "$3"
            ;;
        *)
            echo "Invalid command. Use 'consolidate' or 'recreate'." >&2
            usage
            exit 1
            ;;
    esac
}

# Execute main function
main "$@"