# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- PCI bus enumeration for NVIDIA GPUs (vendor 0x10DE) with BAR decoding and bus mastering
- MMIO register access via BAR0 (NV_PMC, PFIFO, PFB, PGRAPH, PTIMER, PDISP, PMU, SEC2, GSP)
- GPU family detection: Kepler, Maxwell, Pascal, Volta, Turing, Ampere, Ada Lovelace
- Known device ID database covering RTX 20xx, 30xx, and 40xx series
- Falcon microcontroller interface (PMU, SEC2, GSP) with IMEM/DMEM upload, boot, and mailbox
- FIFO command submission: GPFIFO channels, push buffers, runlists, doorbell mechanism
- GPU method encoding (incrementing and non-incrementing)
- Compute class support from Kepler (0xA0C0) through Ada (0xC9C0)
- Shader program loading, constant buffer binding, grid/block dispatch, fence synchronization
- Tensor engine with allocate, upload, download, and free operations
- Tensor operation dispatch scaffolding: MatMul, Softmax, LayerNorm, GELU, SiLU, RoPE
- Data type support: FP32, FP16, BF16, INT8, INT4
- VRAM allocator with GPU page table entries, BAR1 CPU mapping, and DMA system memory mapping
- High-level GpuDevice API tying together all subsystems
- Full `#![no_std]` support with `alloc` dependency only
