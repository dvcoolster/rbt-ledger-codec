# rbt-codec 🎛️

[![build](https://github.com/dvcoolster/rbt-codec/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/dvcoolster/rbt-codec/actions/workflows/build-and-test.yaml)

Next-generation **RBT-powered compression suite** targeting 10–100 × reductions over ZIP/RAR and state-of-the-art video codecs.

---

## Vision & Milestones

1. **`rbtzip` CLI** — loss-less *.rbtz* archives with ≥ 5 × gain on Kodak image set.  
2. **Perceptual video mode** — ≥ 25 × over H.265 at VMAF ≈ 95.  
3. **Browser demo** — drag-and-drop, encode, stream-preview, full restore.  
4. **Lean-verified bounds** — proof that bit-length ≤ emergent-complexity (RBT Cor. 4.3).  
5. **Deterministic builds** — Linux, macOS, Windows & WASM.

Detailed roadmap in [`/docs`](docs/) and [`/specs`](specs/).

---

### Quick start (Rust workspace)

```bash
cargo test --workspace --release
```

Other components (CUDA kernels, web app, Lean proofs) have their own README files.

---

Licensed under MIT (code) – see [`/LICENSE-MIT`](LICENSE-MIT) and Apache-2.0 (model weights). 