# gpu-compute-nostd

[![Crates.io](https://img.shields.io/crates/v/gpu-compute-nostd.svg)](https://crates.io/crates/gpu-compute-nostd)
[![Docs.rs](https://docs.rs/gpu-compute-nostd/badge.svg)](https://docs.rs/gpu-compute-nostd)
[![License](https://img.shields.io/crates/l/gpu-compute-nostd.svg)](LICENSE-MIT)

Bare-metal `#![no_std]` NVIDIA GPU compute driver with tensor operations for LLM inference in Rust. No CUDA runtime, no Linux kernel, no OS dependencies.

## Features

- **PCI enumeration** -- Scans for NVIDIA GPUs (vendor 0x10DE), reads BARs, enables bus mastering via x86 I/O ports
- **MMIO register access** -- Volatile read/write to GPU control registers (NV_PMC, PFIFO, PFB, PGRAPH, PTIMER, PDISP, PMU, SEC2, GSP) via BAR0 mapping
- **GPU family detection** -- Kepler (2012) through Ada Lovelace (2022) with known device ID database covering RTX 20xx/30xx/40xx series
- **Falcon microcontroller** -- Interface for PMU, SEC2, and GSP firmware upload (IMEM/DMEM), boot, mailbox communication, and interrupt handling
- **FIFO command submission** -- GPFIFO channels, push buffers, runlists, doorbell mechanism, and GPU method encoding (incrementing/non-incrementing)
- **Compute dispatch** -- Compute class binding (Kepler through Ada), shader program loading, constant buffer setup, grid/block configuration, kernel launch with fence synchronization
- **Tensor operations** -- MatMul, Softmax, LayerNorm, GELU, SiLU, RoPE dispatch scaffolding for transformer inference
- **VRAM management** -- Bump allocator with GPU page table entries (4K/64K/2M pages), BAR1 CPU mapping, DMA system memory mapping
- **Data types** -- FP32, FP16, BF16, INT8, INT4 tensor element types

## Architecture

```text
+----------------------------------------------------------+
|                    tensor.rs                              |
|  (TensorDescriptor, matmul, softmax, layernorm, GELU)    |
+----------------------------------------------------------+
|                   compute.rs                              |
|  (Compute class setup, shader load, grid dispatch)        |
+----------------------------------------------------------+
|                    fifo.rs                                |
|  (GPFIFO channels, push buffers, runlists, doorbells)     |
+----------------------------------------------------------+
|                   falcon.rs                               |
|  (Falcon microcontroller: PMU, SEC2, GSP-RM firmware)     |
+----------------+-----------------------------------------+
|  memory.rs     |              mmio.rs                     |
|  (VRAM, GPU    |  (NV_PMC, PFIFO, PFB, PGRAPH, etc.)     |
|   page tables, |                                          |
|   DMA mapping) |                                          |
+----------------+-----------------------------------------+
|                  pci_config.rs                             |
|  (PCI vendor 0x10DE detect, BAR mapping, bus mastering)   |
+----------------------------------------------------------+
|                   driver.rs                               |
|  (GpuDevice: high-level init, query, compute API)         |
+----------------------------------------------------------+
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
gpu-compute-nostd = "0.1"
```

### GPU initialization

```rust,no_run
use gpu_compute_nostd::GpuDevice;

// In your bare-metal kernel, after setting up paging and heap:
let gpu = unsafe { GpuDevice::init() };
if let Some(gpu) = gpu {
    let info = gpu.query_info();
    log::info!("Found: {}", info);
    // info.name = "NVIDIA GeForce RTX 3070 Ti"
    // info.family = Ampere
    // info.vram_bytes = 8589934592
    // info.sm_count = 48
}
```

### Tensor allocation

```rust,ignore
use gpu_compute_nostd::{DType, TensorDescriptor};

// Allocate a weight matrix in VRAM
let weights = gpu.tensors.allocate_tensor(
    &gpu.vram,
    "attn_qkv",
    &[4096, 4096],
    DType::Float16,
);
```

### PCI scanning (standalone)

```rust,ignore
use gpu_compute_nostd::pci_config;

if let Some(pci_dev) = pci_config::scan_for_nvidia_gpu() {
    log::info!("Found {} at PCI {:02X}:{:02X}.{}",
        pci_dev.name, pci_dev.bus, pci_dev.device, pci_dev.function);
    log::info!("BAR0 (MMIO): 0x{:X}, BAR1 (VRAM): 0x{:X}",
        pci_dev.bar0_base, pci_dev.bar1_base);
}
```

## Requirements

- **Target**: `x86_64-unknown-none` (or any bare-metal x86_64 target)
- **Allocator**: Requires a global allocator (`#[global_allocator]`) -- uses `alloc` for `Vec`, `String`, etc.
- **Inline assembly**: Uses x86 `in`/`out` instructions for PCI config space access
- **Volatile memory**: Uses `core::ptr::read_volatile`/`write_volatile` for MMIO

## GSP-RM firmware note

Starting with Turing (RTX 20xx), NVIDIA requires signed GSP-RM firmware (~30 MiB) to fully initialize compute engines. Without it, Turing/Ampere/Ada GPUs can be detected and queried but cannot execute compute kernels. Pre-Turing GPUs (Kepler through Pascal) support direct initialization without proprietary firmware.

## Status

This crate provides the complete scaffolding for bare-metal NVIDIA GPU compute. The register layouts, command encoding, and driver flow are based on the [nouveau](https://nouveau.freedesktop.org/) project and [envytools](https://envytools.readthedocs.io/) reverse-engineering work. Actual compute kernel dispatch requires SASS shader binaries compiled for the target GPU architecture.

## References

- [envytools](https://envytools.readthedocs.io/) -- NVIDIA GPU hardware documentation (reverse-engineered)
- [nouveau](https://nouveau.freedesktop.org/) -- Open-source NVIDIA GPU driver for Linux
- [nova](https://gitlab.freedesktop.org/drm/nova) -- New Rust-based kernel driver for NVIDIA GPUs

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
