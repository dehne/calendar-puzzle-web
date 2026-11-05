# Third-party notices and dependency review

Reviewed on 2026-11-04 against the actual locked packages and installed Rust
1.99.1 sources. Our own MIT license does not replace these terms. Retain this
file alongside LICENSE with the deployed HTML/CSS/JS/Wasm assets, including when
copying pkg/ into a deployment. The About & License page links to this document.
Do not distribute pkg/ alone without the accompanying applicable notices.

## Cargo dependencies

The web adapter uses calendar_puzzle 0.1.0 by its unchanged local path. It is
Copyright (c) 2026 D. L. Ehnebuske, MIT, and its notice and terms are also present
in this project's LICENSE. There are no external JavaScript packages, fonts,
images, UI frameworks, or separately licensed palette assets.

Resolved registry packages, checked using cargo metadata/tree and each exact
package's bundled license files:

| Package | Version | Declared license | Role |
|---|---|---|---|
| wasm-bindgen | 0.2.130 | MIT OR Apache-2.0 | Wasm runtime and generated JS glue |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | Runtime configuration macros |
| wasm-bindgen-shared | 0.2.130 | MIT OR Apache-2.0 | Runtime/shared and build code |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | Normal dependency of shared; also used by macros |
| wasm-bindgen-macro | 0.2.130 | MIT OR Apache-2.0 | Build-time procedural macro |
| wasm-bindgen-macro-support | 0.2.130 | MIT OR Apache-2.0 | Build-time macro support |
| bumpalo | 3.20.3 | MIT OR Apache-2.0 | Build-time macro support |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | Build-time procedural macro support |
| quote | 1.0.47 | MIT OR Apache-2.0 | Build-time code generation |
| syn | 3.0.9 | MIT OR Apache-2.0 | Build-time parsing |
| rustversion | 1.0.23 | MIT OR Apache-2.0 | Build-time version check |

MIT is selected for the dual-licensed Cargo packages. Unicode-3.0 is additional,
not an alternative. Preserve its notice even when the code uses the MIT option.
No incompatible or source-disclosure requirement was identified for the reviewed
source/static-web distribution. License choices do not change upstream ownership.

Notices recorded from the exact package LICENSE-MIT files (build-only entries
are retained for audit completeness, not claimed as mandatory runtime notices):

- wasm-bindgen, wasm-bindgen-macro, wasm-bindgen-macro-support,
  wasm-bindgen-shared 0.2.130, and cfg-if 1.0.5:
  Copyright (c) 2014 Alex Crichton.
- bumpalo 3.20.3: Copyright (c) 2019 Nick Fitzgerald.
- proc-macro2, quote, syn, rustversion, and unicode-ident provide MIT permission
  terms without separate copyright lines in LICENSE-MIT. The shared text below
  preserves those terms without inventing notices for them.

The generated bindings contain wasm-bindgen glue, so their upstream MIT notice
is retained here even though generated files receive no project SPDX header.
No third-party source or generated bindings were edited by this milestone.

## Build tools and distribution boundary

- wasm-pack 0.15.1: MIT OR Apache-2.0, verified in its installed Cargo package;
  its MIT file identifies Copyright (c) 2018 Ashley Williams.
- wasm-bindgen CLI 0.2.130: upstream wasm-bindgen MIT OR Apache-2.0. Its generated
  glue is covered by the wasm-bindgen notice above.
- wasm-opt (Binaryen) 130: Apache-2.0, verified against Binaryen's upstream LICENSE.
  It transforms the Wasm module; no Binaryen tool executable/runtime is shipped.
- Rust compiler and linker: build tools, not shipped. The linked Rust runtime is
  addressed separately below. Compiler/tool installation dependency trees are
  not application dependency trees and are not bundled into the website.

No complete Apache license is reproduced merely because an Apache-licensed tool
was used to build the application. Redistributing those tool executables, their
sources, or a different artifact set would require a separate review, including
those tools' own dependencies and notices.

Sources: exact local crate manifests/LICENSE files; installed Rust
COPYRIGHT-library.html and library source licenses;
https://github.com/wasm-bindgen/wasm-bindgen/blob/main/LICENSE-MIT;
https://github.com/WebAssembly/binaryen/blob/version_130/LICENSE;
https://rustc-dev-guide.rust-lang.org/licenses.html.

## Rust runtime (tested stable toolchain 1.99.1)

Compiled Rust applications also use the standard library and compiler runtime,
which are not listed in these applications' Cargo.lock files. The notices below
cover the standard-library inputs supplied with the tested Linux and Wasm
builds; link-time elimination may remove unused portions. They are not claims
that every input survives in every final executable.

MIT is selected where offered as an alternative. Verified notices from the
installed Rust standard-library COPYRIGHT-library.html and source licenses:

- Rust core/alloc/std: The Rust Project Developers
  (https://thanks.rust-lang.org); Apache-2.0 OR MIT.
- std synchronization code: Copyright 2019 The Crossbeam Project Developers;
  Apache-2.0 OR MIT.
- cfg-if 1.0.4, dlmalloc 0.2.14, rustc-demangle 0.1.28:
  Copyright (c) 2014 Alex Crichton; MIT or Apache-2.0.
- hashbrown 0.17.1: Copyright (c) 2016 Amanieu d'Antras;
  MIT OR Apache-2.0.
- memchr 2.8.3: Copyright (c) 2015 Andrew Gallant; Unlicense OR MIT.
- gimli 0.34.0: Copyright (c) 2015 The Rust Project Developers;
  MIT OR Apache-2.0.
- object 0.39.1: Copyright (c) 2015 The Gimli Developers;
  Apache-2.0 OR MIT.
- addr2line 0.27.1: Copyright (c) 2016-2018 The gimli Developers;
  Apache-2.0 OR MIT.
- libc 0.2.189: Copyright (c) The Rust Project Developers;
  MIT OR Apache-2.0. This Rust crate is distinct from the operating system's C
  library; system libraries/tool executables are not bundled by these projects.
- adler2 2.0.1: 0BSD OR MIT OR Apache-2.0; MIT is selected. Its LICENSE-MIT
  supplies the permission terms without a separate copyright line.
- miniz_oxide 0.9.1: MIT OR Zlib OR Apache-2.0; MIT is selected, with notices:
  Copyright 2013-2014 RAD Game Tools and Valve Software;
  Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC;
  Copyright (c) 2017 Frommi; Copyright (c) 2017-2024 oyvindln.
- Rust core Unicode data: Copyright 1991-2024 Unicode, Inc; Unicode-3.0.
  The Unicode terms below apply independently of the MIT choice for Rust code.

compiler_builtins 0.1.160 declares
`MIT AND Apache-2.0 WITH LLVM-exception AND (MIT OR Apache-2.0)`.
Its inherited compiler-rt notice is:
Copyright (c) 2009-2016 by the contributors listed in CREDITS.TXT
(https://github.com/llvm/llvm-project/blob/main/compiler-rt/CREDITS.TXT).
Its original-work MIT alternative is selected where available. The LLVM object
exception below permits distribution of runtime portions embedded by compilation
without Apache sections 4(a), 4(b), and 4(d). This review covers compiled runtime
portions, not redistribution of compiler-builtins source or tool executables.
The full upstream terms are at
https://github.com/rust-lang/compiler-builtins/blob/main/LICENSE.txt.

The shared MIT and Unicode terms below accompany these notices. Recheck the
runtime notices when changing Rust versions, targets, or binary packaging; this
review does not inventory operating-system libraries or a redistributed compiler.

### LLVM object-code exception (verbatim from the installed runtime)

```text
---- LLVM Exceptions to the Apache 2.0 License ----

As an exception, if, as a result of your compiling your source code, portions
of this Software are embedded into an Object form of such source code, you
may redistribute such embedded portions in such Object form without complying
with the conditions of Sections 4(a), 4(b) and 4(d) of the License.

In addition, if you combine or link compiled forms of this Software with
software that is licensed under the GPLv2 ("Combined Software") and if a
court of competent jurisdiction determines that the patent provision (Section
3), the indemnity provision (Section 9) or other Section of the License
conflicts with the conditions of the GPLv2, you may retroactively and
prospectively choose to deem waived or otherwise exclude such Section(s) of
the License, but only in their entirety and only with respect to the Combined
Software.
```

## Shared MIT permission terms

The following permission and disclaimer apply to the MIT-licensed components
identified above, together with their respective copyright notices. Components
whose upstream MIT file has no copyright line are not assigned invented holders.

```text
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```

## Unicode License V3

The additional Rust core notice is Copyright 1991-2024 Unicode, Inc.
The notice below is reproduced from unicode-ident 1.0.27; its terms also cover
the Rust core Unicode data identified above.

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```
