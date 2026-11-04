set default-list

run: build serve

fmt:
    cargo fmt --all
    pnpm fmt
    tombi fmt

lint:
    cargo lint
    tombi lint

build:
    cargo run --release --package site
    pnpm build:assets

serve:
    miniserve dist --port 3000 --index index.html --header "Cache-Control:no-cache"

clean:
    rm -rf .cache dist

pull-queries:
    cargo xtask query pull

update-queries:
    cargo xtask query update
