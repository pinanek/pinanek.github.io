+++
title = "Introducing ContextCraft: code context on a token budget"
description = "How ContextCraft uses syntax trees, lexical search, and dependency ranking to fit useful code context into an LLM’s token budget."
pubDate = "2026-11-04T22:10:36Z"
tags = ["blog", "rust", "algorithms"]
+++

You want an LLM to help with authentication in your repository. Which files do you give it? The authentication directory is a good start, but the session model might live somewhere else, and the configuration code could matter too. Paste the whole repository and you could use up the context window before you even get to your question.

[ContextCraft](https://github.com/khanhtq29vn/contextcraft) helps you pick what to include. Give it a repository, a task, and a token budget, and it builds a code context document to fit. It uses your task and the relationships between files to decide which files get their full source, which get an outline, and which get just a path entry.

The project is by [Khanh Quoc Tran](https://github.com/khanhtq29vn), a PhD candidate in Computer Science at the NUS School of Computing working on trustworthy AI, ML systems, and causal reasoning. He’s a great guy. I’ve been involved since the early Python days and later moved the core to Rust :). Before getting into that story, let’s try the tool.

## Give it a repository and a task

You can try the CLI with `uvx` without installing it permanently:

```console
uvx --from 'contextcraft-ai[cli]' contextcraft . \
  --budget 8000 \
  --focus "authentication" \
  -o context.md
```

This reads your current repository and writes `context.md` with an 8,000-token budget. Open it to see what made the cut before passing it to an LLM. You can also choose XML or JSON output instead of Markdown.

The focus query tells ContextCraft what you’re working on. If you already know which part of the repository matters, you can use `--focus-file` to prefer a path, directory, or glob, such as `--focus-file 'src/auth/**'`. That gives matching files a boost in the ranking, but doesn’t guarantee they’ll all fit or get their full source included.

Git ignore rules and include/exclude patterns control what gets scanned. If you enable secret redaction, it runs before source analysis. Still, please review the output before sharing it: redaction can miss credentials.

## Keep the interface, shorten the implementation

You don’t have to choose between including a whole file and leaving it out. ContextCraft can give each file one of four levels of detail, called fidelity tiers:

| Tier   | What it retains                                                 |
| :----- | :-------------------------------------------------------------- |
| `FULL` | Complete loaded source after optional secret redaction          |
| `SKEL` | A richer outline retaining structure and documentation          |
| `SIGS` | A compact declaration outline                                   |
| `TREE` | No source block, with a path entry if the listing budget allows |

For our authentication task, that might mean keeping the main authentication logic in full, including just the declarations from a related model, and listing unrelated files by path. The exact mix depends on your repository, the file rankings, and how much room is left in the budget.

To build those outlines, ContextCraft uses Tree-sitter syntax trees. Language-specific queries identify the byte ranges containing declarations and documentation, and the renderer uses them to put together the shorter versions. That lets it keep the structure instead of cutting a file off after an arbitrary number of lines.

The outline doesn’t need to run as code. It just needs to show useful structure in less space. If ContextCraft doesn’t have a matching language implementation for a file, the compressed tiers can fall back to its full readable source.

## How does it choose the relevant files?

ContextCraft looks for words that match your task, then follows dependencies to find related files that might use different words. It combines lexical search with a dependency graph to do this.

### Search across more than the file body

The lexical scorer uses BM25F, a field-weighted form of BM25. It searches four fields:

| Field                    | Weight |
| :----------------------- | -----: |
| Repository-relative path |    5.0 |
| Declaration signatures   |    4.0 |
| Documentation            |    2.5 |
| Full source              |    1.0 |

So a match in a filename or function signature counts more than the same word buried in an implementation. Long fields don’t win just because they contain more text: field-length normalization accounts for their size. Repeating a word has diminishing returns too, thanks to term-frequency saturation.

Identifier splitting helps too. `SessionToken` can contribute `sessiontoken`, `session`, and `token`, connecting ordinary query words to names in code without an embedding service.

There are two kinds of tokens here. These lexical tokens help score relevance. BPE tokens[^bpe] measure how much space the final document takes.

### Follow local dependencies

For each file, ContextCraft adds a directed edge to every local dependency it can resolve. Language-specific resolvers find these connections through static analysis, so the graph won’t capture everything that happens at runtime.

The scorer starts with a preference for each file:

$$
p_0(i) = 1 + \operatorname{focused}(i) + 3.5\,\operatorname{lexical}(i)
$$

Here, `focused` is one when the file matches a configured focus pattern and zero otherwise. Strong preferences can spread one hop to dependencies before being normalized into a personalization distribution.

Personalized PageRank then repeatedly passes rank along dependency edges. With a damping factor of 0.85, part of each update follows the graph and the rest returns to the task-biased distribution. Files with no outgoing edges redistribute their rank through that same distribution.

Back to our session model: it can gain relevance even if it never mentions “authentication”, because the focused authentication code depends on it. The word matches give us a starting point, and the graph helps us find the surrounding code.

Each PageRank update does work proportional to the number of files plus dependency edges. It stops when the total change in rank is small enough, or after 200 updates.

## How much detail can we afford?

Now that the files have scores, ContextCraft needs to decide how much of each one to include. Where will another token buy the most useful detail?

Each tier has a relative utility: 0 for `TREE`, 0.45 for `SIGS`, 0.82 for `SKEL`, and 1 for `FULL`.[^tier-utility]

For a possible upgrade, the allocator compares the extra score-weighted utility with the extra token cost:

$$
\operatorname{density} =
\frac{\operatorname{rank}(i)\,[u(\text{next})-u(\text{current})]}
{C(i,\text{next})-C(i,\text{current})}
$$

The allocator keeps these possible upgrades in a max-heap, with the best value per token at the top. If an upgrade fits, it takes it and considers that file’s next best upgrade. Sometimes it skips an intermediate tier because a bigger jump offers better value.

Counting tokens for every version of every file would mean a lot of upfront work. Instead, the allocator starts with estimates based on byte counts. It counts the selected tiers exactly, caches those costs, and reruns allocation when it has a more accurate cost.

In algorithm terms, this is a greedy heuristic for a multiple-choice knapsack problem, so it won’t necessarily find the best possible selection.

### The budget includes the wrapping

Source text isn’t the only thing using tokens. Paths, labels, escaping, and Markdown, XML, or JSON framing count too. Even exact counts of individual blocks don’t necessarily add up to the count of their concatenation, because BPE merges can cross block boundaries.

To account for all of that, the formatter renders the complete document and counts its tokens. If it’s over budget, it lowers fidelity where it loses the least utility per token saved, then renders and counts again. It can shorten the path listing too. If even the minimal framing won’t fit, it returns an error rather than a document that’s too large.

When it succeeds, the document fits the budget for your selected tokenizer, though it may leave some tokens unused. Keep in mind that a different model’s vocabulary may give you a different count.

## Use it from Python too

The CLI is one way to use ContextCraft. You can also call the same pipeline from Python:

```python
from contextcraft import Config, ContextCraft

pipeline = ContextCraft(
    Config(
        budget_tokens=8000,
        focus_query="authentication",
    )
)
analysis = pipeline.run(".")

print(analysis.artifact.tokens)
print(analysis.artifact.content)
```

You also get the intermediate data: the index, loaded sources, dependency graph, scores, rendered tiers, and final selection. If you’re wondering why a file didn’t make it into the context, you can look through those stages to see what happened.

Each `run()` rereads and analyzes the repository. The core runs in Rust, exposed through PyO3 bindings that release the Python GIL during processing. Source analysis and tier rendering use Rayon for parallel execution by default.

## Working on the early Python version

I started working on ContextCraft on October 22, 2026, the day the repository was created. Back then, it was all Python. That included language-specific AST extractors and [a language-pipeline rewrite in PR #19](https://github.com/khanhtq29vn/contextcraft/pull/19), which organized 33 supported languages into dedicated extractor and builder packages and exposed custom language registration through Python.

Then came [PR #23](https://github.com/khanhtq29vn/contextcraft/pull/23): BM25F focus scoring, dependency-ranking improvements, and better token-budget allocation. I also worked on speeding up allocation and reusing repository analysis, so the Python pipeline could analyze once and render at different budgets without scanning and parsing everything again.

By the time of [the Python implementation just before the migration](https://github.com/khanhtq29vn/contextcraft/tree/61567d4e811dd5eff225df377a0769cc414a3c3e), the central ideas were already there: Tree-sitter analysis, BM25F, personalized PageRank, and fidelity allocation. That Python work gave us the starting point for the Rust version.

## Moving the engine to Rust

The first Python version was still unoptimized. We’d made it faster, and there was more we could do in Python, but I decided to move the core to Rust at that point. We hadn’t reached Python’s performance limit. I chose to put the next round of work into Rust.

[PR #34](https://github.com/khanhtq29vn/contextcraft/pull/34), merged on November 1, 2026, moved the compression engine into `contextcraft_core` and exposed it through PyO3 bindings. Python remained the entry point for the SDK and CLI, while Rust handled indexing, source analysis, dependency resolution, ranking, token counting, and allocation.

The parallel stages are source analysis and tier rendering, using Rayon. The Python binding releases the GIL while the main pipeline runs, letting that work happen outside the Python interpreter. The migration also included buffer reuse and preallocated file storage rather than relying on the language change alone.

### What changed in the algorithms?

I changed some of the algorithms during the migration too, so the two versions do slightly different work. That matters when we get to the benchmark.

The Python scorer used a NetworkX directed multigraph. Imports contributed weighted edges, unresolved symbol references could connect files to symbol definitions, and weaker reverse edges let relevance travel back toward callers. Filename priors favored some architectural files, while tests were penalized unless the focus called for them.

The Rust graph instead uses resolved local dependency edges, with duplicates collapsed and self-dependencies excluded. It uses the simpler initial preference described above rather than carrying over those filename priors and test penalties.

Allocation changed too. After its greedy upgrades, the Python optimizer tried bounded pairwise swaps: could lowering one file’s fidelity free up enough tokens for a more valuable upgrade elsewhere? The Rust allocator doesn’t retain that swap phase. It uses greedy allocation with lazy token-cost refinement, counting selected tiers and revisiting the allocation as better costs become available. Both versions check the final serialized output against the budget.

The PR also moved documentation from Sphinx to Astro and Starlight, added generated Rust and Python API references, and updated CI and packaging. There was quite a bit around the core engine that needed to move with it.

## The benchmark, on an older Xeon

I ran the comparison on an **Intel Xeon E5-2660 v4 @ 2.00 GHz**. It’s an older server CPU, so keep that in mind when looking at the timings.

The [PR’s performance comparison](https://github.com/khanhtq29vn/contextcraft/pull/34) used Home Assistant’s repository and included three versions: the original Python implementation, the faster Python `next` version, and the Rust core.

| Metric          | Original Python | Faster Python (`next`) |         Rust |
| :-------------- | --------------: | ---------------------: | -----------: |
| Wall-clock time |      1,753.21 s |               126.91 s |      7.214 s |
| Peak RSS        |   53,015.63 MiB |             925.76 MiB | 1,548.29 MiB |
| CPU utilization |             95% |                   269% |      1083.4% |
| Scanned files   |          29,965 |                 30,018 |       30,018 |

The Python work had already brought runtime down from 1,753.21 seconds to 126.91 seconds, with a large reduction in peak memory. A lot of the improvement came before the Rust rewrite, with room left to tune Python further.

The Rust figures are the mean of five runs. Compared with the faster Python version, Rust was about 17.59 times faster by wall-clock time but used about 1.67 times the peak memory. It also used more CPU parallelism. CPU utilization can exceed 100% when several cores are active.

I’m using the measurements recorded in the PR here. I haven’t rerun the benchmark for this post. The original Python run scanned fewer files, so its workload wasn’t identical. The faster Python and Rust versions scanned the same number, but their ranking and allocation differences mean they may have selected different content or levels of detail. These timings are for this repository on this machine. Your results will depend on the workload.

## Give it a try

If you’re putting together code context for an LLM, try the command above on a repository you know. Inspect which files get full source and which get outlines, then change the focus or budget and compare the result. The intermediate scores and tiers are there if you want to dig into why a selection changed.

You can find [ContextCraft on GitHub](https://github.com/khanhtq29vn/contextcraft), read the [documentation](https://khanhtq29vn.github.io/contextcraft/), or browse [the migration PR](https://github.com/khanhtq29vn/contextcraft/pull/34).

[^bpe]: BPE stands for byte-pair encoding. It represents text using tokens that can be words, parts of words, or other byte sequences. The count depends on the tokenizer’s vocabulary, not just the number of words or characters.

[^tier-utility]: These are heuristic weights for allocation, not measured probabilities that the model will find something useful.
