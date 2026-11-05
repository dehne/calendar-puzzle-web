# Calendar Puzzle Web — W2

An interactive calendar puzzle solver running the existing Rust library in the
browser through WebAssembly. Choose weekday, month, and day, then click **Solve**
to display a colored 9 × 7 pentomino tiling, search-node count, and solve time.
Every successful result is independently validated in Rust before display.

The initial selectors use the browser's **local date**. After initialization,
weekday, month, and day are independent. Month changes update the day options and
clamp an out-of-range day to that month's last day. February always offers 29 days.
All 2,562 valid calendar combinations are available. Selecting a different date
clears the previous board until Solve is clicked again.

## Project layout

```text
calendar-puzzle-web/
├── Cargo.toml / Cargo.lock   # Wasm wrapper; no shared Cargo workspace
├── src/lib.rs               # Retained layout, solve interface, native tests
├── layouts/two-swaps.txt    # Static configuration fetched at initialization
├── index.html               # Accessible date controls and board container
├── app.js                   # Load Wasm/layout, manage controls, render results
├── style.css                # Responsive square grid and twelve piece colors
├── VERIFICATION.md          # Actual build/browser results and measured times
├── pkg/                     # Generated bindings and Wasm (Git-ignored)
└── target/                  # Cargo build output (Git-ignored)
```

There are no JavaScript frameworks, bundlers, Node.js dependencies, or server-side
solves. Rust runs synchronously on the browser's main thread. The page has no
layout chooser, upload control, solution cache, search limits, or timeouts.

## Dependency arrangement

The original repository has no configured Git remote, so the original crate is
used directly without copying source:

```toml
calendar_puzzle = { path = "../calendar_puzzle" }
```

Expected sibling directories:

```text
Projects/
├── calendar_puzzle/      # Original CLI/library repository, unchanged
└── calendar-puzzle-web/  # This independent repository
```

The tested library baseline is commit
`c7704820395e1e3b1d02dda7eb3dd55f0b553360`. The path dependency follows the local
checkout rather than pinning it. Only the two-swaps **configuration data** is
copied into this project; it matches the original `layouts/two-swaps.txt` exactly.
There is no solver source duplication. The only additional direct Rust dependency
is `wasm-bindgen`; W2 adds no dependencies.

When a remote becomes available, replace the path entry with the following,
substituting the actual URL and a full commit hash that includes the MIT license
and publication notices added after that baseline:

```toml
calendar_puzzle = { git = "https://HOST/OWNER/REPOSITORY.git", rev = "FULL_COMMIT_WITH_MIT_LICENSE" }
```

Then run `cargo update -p calendar_puzzle`, keep the updated lockfile, and repeat
the tests and Wasm build. No commits have been made in the web repository.

## Install, build, and serve

Install stable Rust and Python 3 if necessary. From this project directory:

```bash
rustup toolchain install stable
rustup override set stable
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked

cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
wasm-pack build --target web --release
python3 -m http.server 8000 --bind 127.0.0.1
```

Open **http://localhost:8000/**. Wait for the ready message, choose a date, and
click Solve. Stop the server with Ctrl+C. Initial dependency/tool downloads need
network access. `wasm-pack` supplies its matching wasm-bindgen CLI automatically.
Build before serving; `pkg/` is generated and intentionally ignored by Git.
Do not load `index.html` using `file://`.

All asset URLs are relative: `./app.js`, `./style.css`,
`./layouts/two-swaps.txt`, and `./pkg/calendar_puzzle_web.js`. The generated module
loads its Wasm file relative to itself, so a repository URL subpath also works.
Rebuild after changing Rust code; refresh after changing static files.

## Rust/JavaScript interface

After `await init()`, JavaScript constructs `new CalendarSolver(layoutText)`.
The constructor uses the original `CalendarLayout::parse()` and retains that
validated layout. Layout HTTP and parsing failures leave Solve disabled and
show a visible error. The external file is fetched once, not on each solve.

The small Wasm interface is:

- `solver.weekdays()` and `solver.months()` return the original library's labels.
- `solver.days_in_month(month)` returns its leap-year month length. The controls
  use this data rather than maintaining a second calendar rule table in JS.
- `solver.solve(weekday, month, day)` accepts abbreviated labels and a day string
  such as `solver.solve("Sat", "Oct", "31")`. The original `CalendarDate::parse()`
  is authoritative, including for invalid requests made outside the controls.
- The returned `CalendarSolution` has `solved: boolean`, `cells: string[]`, and
  `nodes_visited: bigint`. A solved result contains 63 row-major strings: 60
  pentomino letters and the three selected calendar labels. An unsolvable result
  has `solved === false` and an empty cell array. Errors throw into JavaScript.

Rust constructs the board from the retained layout, calls `solve_with_stats()`,
and calls the independent `validate_solution()` before serializing placements.
Only display cells are transferred; placement masks and search structures remain
inside Rust. The original W1 fixed-date export is replaced by this interface.

JavaScript copies the returned display data and calls `result.free()` in a
`finally` block. The node count is a BigInt to preserve the library's exact u64
value; display it as text, without converting it to a floating-point Number.
The retained solver lives for the page lifetime. No solve results are cached.

Timing uses `performance.now()` around the Wasm call **and result getters**,
ending before DOM construction and styling. Browser timer precision and JIT
warm-up affect measurements. No Rust native clock is invoked by the wrapper.

The page creates cells with DOM operations and `textContent`, using a stable CSS
class for each pentomino. The three uncovered cells use white backgrounds and
show their actual weekday/month/day labels. Errors and `UNSOLVABLE` results clear
the previous board; invalid requests do not become unsolvable results.

## Verification and manual checks

Run the native checks above. They cover the retained W1 conversion/validation
properties, selected-label coordinates, representative dates, month lengths,
invalid inputs, and proven-unsolvable behavior. They require no browser.
The original CLI tests can be checked without placing build output in that repo:

```bash
cargo test --locked --manifest-path ../calendar_puzzle/Cargo.toml --target-dir /tmp/calendar-original-tests
```

Browser checks:

1. Reload. Confirm the initial selectors match today's local weekday/month/day;
   Solve stays disabled until Wasm and the layout are ready. There is no automatic
   solve on load.
2. Select Saturday, January 31, then change to April. Expect April 30 with Saturday
   unchanged. Change to February: expect 29 days, no February 30, and the same
   weekday. Changing the day must also leave the weekday unchanged.
3. Solve Sat Oct 31, Mon Jan 1, Fri Jun 30, and Wed Feb 29. Each should show
   `SOLVED`, search nodes, milliseconds, 63 square cells, five of each piece, and
   exactly three labeled white cells. For Sat Oct 31 these are `(8,0)`, `(5,2)`,
   and `(8,6)`. Check the same piece keeps the same color across solves.
4. Check a narrow viewport (320px and wider), keyboard navigation, and visible
   focus. The controls wrap and the board keeps its 9:7 proportions.
5. In developer tools, block the layout, generated JavaScript, or Wasm request
   and reload with caching disabled. Expect a visible error, empty board, and
   disabled Solve. Unblock and reload to recover.
6. To test Rust rejecting an invalid request through the form after a valid
   solve, run this in the browser console:

   ```js
   document.getElementById("day").value = "";
   document.getElementById("solve").click();
   ```

   Expect an error and an empty board; select a valid day and Solve to recover.
7. Serve the parent directory and open `/calendar-puzzle-web/` to verify relative
   assets under a repository subpath.

The shipped two-swaps arrangement solves every valid date. `UNSOLVABLE` handling
is tested natively using the original arrangement and in the browser using a
verification-only HTTP response fixture; no layout-selection UI is added.
See [VERIFICATION.md](VERIFICATION.md) for actual results and compatibility limits.

## Development

This project was developed with substantial assistance from OpenAI's ChatGPT and Codex. The project requirements, architecture, design decisions, and acceptance criteria were directed and reviewed by the project maintainer. Codex assisted with implementation, testing, and refactoring.

## License

Copyright (c) 2026 D. L. Ehnebuske.

This project is distributed under the MIT License. See [LICENSE](LICENSE) for details.

Third-party components retain their own licenses. See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for the dependency review and
notices to retain with distributed binaries or browser assets.
