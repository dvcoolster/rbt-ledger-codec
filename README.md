# RBT-Ledger-Codec

> **TL;DR (10 lines)**
> 1. ▸ *Ledgerizer* (C++17) = reversible even/odd loop graph over bytes.  
> 2. ▸ *FlowNet* (PyTorch) = invertible conditional flow on phase-tags.  
> 3. ▸ *ANS-X* (Rust/C++) = SIMD RANS coder on FlowNet latents.  
> 4. ▸ *Percep* (PyTorch) = optional perceptual enhancer / task head.  
> 5. Goal: **×100 compression** on images/video while remaining mathematically loss-less at L0.  
> 6. Alpha target in **3 months**.  
> 7. This repo starts with **Sprint S₀** – *ledgerizer/*.  
> 8. All code is MIT-licensed & open-source.  
> 9. We favour **small focused modules**; see */docs/* for full spec.  
> 10. Pull requests & design feedback welcome!

---

## Project structure

| Layer | Directory | Status |
|-------|-----------|--------|
| L0 – Ledgerizer | [`ledgerizer/`](ledgerizer/) | 🚧 active |
| L1 – FlowNet | `flownet/` | not yet |
| L2 – ANS-X | `ansx/` | not yet |
| L3 – Perceptual | `percep/` | not yet |

Detailed sprint plan lives in [`PROJECTS/ROADMAP.md`](docs/ROADMAP.md) *(coming soon)*.

## Local build quick-start

```bash
# deps: cmake ≥3.18, C++17 compiler
mkdir build && cd build
cmake .. && cmake --build . -j4
ctest # run unit tests
```

See [`ledgerizer/README.md`](ledgerizer/README.md) for running the L0 demo.

## Repository guidelines

* Small self-contained modules with clear APIs.
* Follow [Google C++](https://google.github.io/styleguide/cppguide.html) or PEP-8 for Python.
* Keep docs in **multiple short markdown files** with relative links.
* No emojis in headings; unicode arrows (→) & subscripts (₀) acceptable.

---

© 2025 