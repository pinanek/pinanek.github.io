#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
    printf 'Usage: %s DIRECTORY\n' "$0" >&2
    exit 1
fi

for command in exiftool jpegoptim; do
    if ! command -v "$command" >/dev/null 2>&1; then
        printf 'Missing required command: %s\n' "$command" >&2
        exit 1
    fi
done

source_dir=$1

if [ ! -d "$source_dir" ]; then
    printf 'Not a directory: %s\n' "$source_dir" >&2
    exit 1
fi

processed=0
skipped=0

while IFS= read -r -d '' source_path; do
    timestamp=""

    for tag in DateTimeOriginal CreateDate; do
        timestamp=$(exiftool -s3 -d '%m-%d-%Y_%H%M%S' "-$tag" "$source_path")

        if [ -n "$timestamp" ]; then
            break
        fi
    done

    if [ -z "$timestamp" ]; then
        printf 'Skipping %s: no EXIF capture date found\n' "$source_path" >&2
        skipped=$((skipped + 1))
        continue
    fi

    directory=$(dirname "$source_path")
    destination="$directory/$timestamp.jpg"
    suffix=1

    while [ -e "$destination" ] && [ "$destination" != "$source_path" ]; do
        destination="$directory/${timestamp}_$suffix.jpg"
        suffix=$((suffix + 1))
    done

    if [ "$source_path" != "$destination" ]; then
        mv "$source_path" "$destination"
    fi

    jpegoptim --max=85 --all-progressive --preserve "$destination"
    processed=$((processed + 1))
done < <(find "$source_dir" -type f \( -iname '*.jpg' -o -iname '*.jpeg' \) -print0)

printf 'Processed %d JPEG(s); skipped %d.\n' "$processed" "$skipped"
