set default-list

run: build serve

fmt:
    cargo fmt
    pnpm fmt
    tombi fmt

lint:
    cargo lint
    tombi lint

build:
    cargo run --release

serve:
    miniserve dist --port 3000 --index index.html --header "Cache-Control:no-cache"

clean:
    rm -rf .cache dist

pull-queries:
    #!/usr/bin/env bash
    set -euo pipefail

    helix_repo="https://github.com/helix-editor/helix.git"
    helix_ref="master"
    helix_cache=".cache/helix"
    queries_dir="queries"

    trap 'rm -rf "$helix_cache"' EXIT

    rm -rf "$helix_cache"

    git clone \
        --depth 1 \
        --filter=blob:none \
        --sparse \
        --branch "$helix_ref" \
        "$helix_repo" \
        "$helix_cache"

    git -C "$helix_cache" sparse-checkout set runtime/queries

    mkdir -p "$queries_dir"

    # Remove old generated queries while preserving non-SCM files.
    find "$queries_dir" -type f -name '*.scm' -delete
    find "$queries_dir" -mindepth 1 -type d -empty -delete

    cp -R "$helix_cache/runtime/queries/." "$queries_dir/"
