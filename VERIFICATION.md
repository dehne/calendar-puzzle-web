# W2 verification

Verified on 2026-11-04 with stable Rust 1.99.1, wasm-pack 0.15.1,
wasm-bindgen 0.2.130, and Firefox 158.0 on Linux.

## Rust and build checks

- `cargo test --locked --offline`: all **7** web-project tests passed. W1's
  three computation/encoding/rejection tests were adapted to the structured
  result; four tests add representative solves, invalid layout/date inputs,
  library-supplied selector metadata, and unsolvable/error recovery.
- `cargo fmt --check`: passed.
- `cargo clippy --locked --offline --all-targets -- -D warnings`: passed.
- `wasm-pack build --target web --release`: passed. The tool required access to
  its existing wasm-bindgen cache outside the sandbox; no source or dependency
  changes were needed for Wasm compatibility.
- Original library/CLI tests, with build output directed to
  `/tmp/calendar-w2-original-test-target`: all **43** passed.
- `cmp layouts/two-swaps.txt ../calendar_puzzle/layouts/two-swaps.txt`: identical.
- Original CLI repository status remained clean. No commits were made.

The Rust tests reconstruct placements from representative result cells and run
`validate_solution()` independently. All successful application results also pass
that validator before serialization. Exactly 63 cells, 60 occupied cells, five
of each pentomino, and the three date labels are checked. Sat Oct 31 still visits
1,251 nodes. Invalid requests remain errors; original-layout Mon Jan 1 remains
proven unsolvable in one search node with an empty result array.

## Actual browser checks

A temporary Python-standard-library HTTP harness launched installed Firefox
headlessly, served the actual application under `/calendar-puzzle-web/`, and
injected a test module that operated the real selectors and Solve button. There
was no replacement solver, added browser framework, or persistent testing
service. These tests are outside the normal Rust suite; manual reproduction
steps are in README.md.

All eight browser scenarios passed:

1. **Desktop interaction:** initial selectors matched the browser's real local
   date (Wed Nov 4). Solve started disabled, became enabled only after Wasm and
   layout validation, and did not execute automatically. All 2,562 combinations
   were available through 7 weekdays and 366 month/day choices. Only the four
   representative dates plus error recovery were solved, not the full calendar.
2. **Narrow viewport:** the same interaction checks passed in a 320px browser
   iframe viewport. The board was 288px wide, stayed at 9:7, had square cells, and
   produced no horizontal page overflow. Desktop board width was 672px.
3. **Local date boundary:** with a test-only frozen instant of
   `2025-01-01T01:30:00Z` and browser timezone America/Los_Angeles, the selectors
   correctly showed **Tue Dec 31**, rather than the UTC date Wed Jan 1.
4. **Layout HTTP failure:** deliberate HTTP 503 produced a visible error, empty
   board, and disabled controls.
5. **Invalid layout:** an unrecognized token containing HTML-like text was
   rejected by Rust, displayed literally in the error, and never interpreted as
   HTML. Controls remained disabled.
6. **Wasm HTTP failure:** deliberate HTTP 503 produced a visible initialization
   error, empty board, and disabled controls.
7. **Bindings HTTP failure:** a failed dynamic import was reported visibly with
   the board empty and controls disabled.
8. **Unsolvable fixture:** the verification server supplied the original layout
   data instead of the default, solely for this test. After a successful solve,
   Mon Jan 1 displayed `UNSOLVABLE`, one node, and no stale cells. A subsequent
   valid solve succeeded. The application has no alternate-layout control.

The interactive scenarios additionally verified:

- January 31 changes to April 30, then February 29; invalid days are absent.
- Month and day changes never modify the selected weekday.
- Every successful board has 63 cells, exactly 60 pentomino cells, and correct
  selected labels at the expected coordinates.
- Each pentomino has five cells of one color; all twelve colors are distinct
  and remain stable across solves.
- Rust rejects an invalid date submitted through the form, clears the earlier
  solution, restores the Solve button, and permits a later valid solve.
- Changing any selection clears the previous solution and returns to ready.
- The default layout is fetched once and retained for multiple solves.
- Relative CSS, layout, JavaScript, and Wasm asset URLs work under the subpath.

## Representative browser measurements

The displayed timer surrounds the Wasm solve and result transfer, ending before
DOM construction. These are individual headless-browser observations, not a
benchmark; JIT warm-up, scheduling, and browser timer rounding affect them. The
"mobile" row describes a narrow Firefox viewport, not physical mobile hardware.

| Browser scenario | Date | Search nodes | Elapsed ms |
|---|---|---:|---:|
| interactive | Sat Oct 31 | 1,251 | 35.00 |
| interactive | Mon Jan 1 | 2,826 | 32.00 |
| interactive | Fri Jun 30 | 379 | 6.00 |
| interactive | Wed Feb 29 | 960 | 37.00 |
| mobile | Sat Oct 31 | 1,251 | 14.00 |
| mobile | Mon Jan 1 | 2,826 | 23.00 |
| mobile | Fri Jun 30 | 379 | 7.00 |
| mobile | Wed Feb 29 | 960 | 12.00 |

The repeated desktop Sat Oct 31 recovery solve took 23.00 ms; the narrow-viewport
recovery solve took 12.00 ms. Browser timing was rounded to whole milliseconds in
this environment even though the application uses `performance.now()` and
formats two decimal places.

## Compatibility and next steps

Actual execution was verified in Firefox 158.0 at desktop and narrow viewport
sizes. Chromium, Safari, and physical mobile devices were not available for
verification; no compatibility claims about those specific environments were
inferred from the Firefox tests. The interface uses native ES modules,
WebAssembly, BigInt, CSS Grid, and CSS aspect-ratio.

No solver or Wasm compatibility blocker was found. The solver remains
synchronous on the main browser thread, and the local path dependency still
tracks the neighboring library checkout. Those remain considerations for later
milestones. W2 adds no dependencies, workers, result caching, alternate layout
selection, deployment, or full browser calendar evaluation.


## W3 publication-preparation verification — 2026-11-04

Both projects now identify their own code as MIT, with Copyright (c) 2026
D. L. Ehnebuske, SPDX identifiers on maintained source, and Development/License
README sections. THIRD_PARTY_NOTICES.md records the actual resolved Cargo
licenses, tool/runtime review, selected alternatives, and retained notices.
The package descriptions were added without inventing repository URLs. The
web dependency remains the same local path, and both Cargo.lock files are
unchanged.

Verification passed:

- CLI: all 43 tests, formatting, and Clippy with warnings denied.
- Web: all 7 tests, formatting, and Clippy with warnings denied.
- `wasm-pack build --target web --release` in a temporary verification copy.
  Test targets were also directed to /tmp, keeping existing output out of the
  verification builds. All existing generated pkg/ files remain byte-for-byte
  unchanged. Background/editor checking refreshed several ignored target/
  diagnostic and metadata cache files while the source headers changed; these
  are not published web assets.
- All 18 maintained Rust/test/JavaScript/CSS/HTML files have the proper SPDX
  identifier. Aside from those comments, Rust and application JavaScript match
  their pre-W3 content; the Wasm interface and algorithms are unchanged.
- Both root LICENSE files are identical standard MIT texts with the specified
  holder and year. The static About page displays those same terms verbatim.
- All eight Firefox W2 regression scenarios passed again using the freshly
  built temporary Wasm package. This includes the four representative dates,
  exact node counts, mobile sizing, date controls, loading/validation errors,
  and proven-unsolvable recovery.
- Actual footer-link navigation opened `/calendar-puzzle-web/about.html`.
  Its copyright, license terms, acknowledgment, and relative links to LICENSE
  and THIRD_PARTY_NOTICES.md were accessible under that repository subpath.

No publication, GitHub Pages configuration, Git tags, commits, dependency changes,
or dependency-license tooling were introduced. Before deployment, include the
root LICENSE and THIRD_PARTY_NOTICES.md with the static assets and generated
Wasm/bindings. Future Git dependency pins should contain these MIT additions,
rather than pointing only to the earlier functional baseline.
