# Roadmap

| Sprint | Duration | Deliverable | Key repos/libs |
|--------|----------|-------------|----------------|
| **S₀** | 2 wks | `ledgerizer/` C++17 library: byte-stream ↔ even/odd loop graph | `fmt`, `range-v3` |
| **S₁** | 4 wks | `flownet/` PyTorch invertible ResNet with phase-tag conditioning | FrEIA, bitsandbytes |
| **S₂** | 3 wks | `ansx/` Rust or C++ fast SIMD RANS, adaptive tables API | rav1e ANS fork |
| **S₃** | 4 wks | `percep/` Dual-head enhancement: ESRGAN-lite (human) or DETR-lite (AI) | PyTorch + ONNX |
| **S₄** | hardening | FFmpeg plugin + `.rbt` container spec (Matroska-ext) | FFmpeg contrib |
| **S₅** | publish | Preprint + demo site (drag-drop MP4/PNG) | HuggingFace Spaces |

---

These dates are **targets**, not guarantees.  Track detailed tasks in GitHub Issues & Projects. 

## Progress

- [x] CLI skeleton (encode/decode PNG round-trip) – v0.1.1
- [x] ANS-X stub crate with C FFI integration – v0.2.0
- [x] ANS stub done – CI green 