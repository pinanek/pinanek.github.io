# Tree-sitter queries

Sources and pinned commits are defined in [`repositories.json`](../repositories.json); language mappings are in [`queries.json`](../queries.json).

- `cargo xtask query pull`: fetch, resolve inheritance, and minify queries.
- `cargo xtask query update`: update pinned commits to remote HEAD. Run `pull` afterward.

Pull queries before building the site. Shared fragments are inlined; only language entries produce output directories.

## Licenses

Helix-derived queries are copyright the Helix contributors and licensed under [MPL-2.0](licenses/helix/LICENSE). Any modifications to these covered source files are distributed under MPL-2.0, and existing copyright and license notices must be preserved.

Containerfile queries come from [tree-sitter-containerfile](https://github.com/wharflab/tree-sitter-containerfile) and are licensed under MIT. They are not covered by the Helix MPL-2.0 license.

Generated files identify their sources and revisions. Upstream license files are copied to `licenses/<repository>/`.
