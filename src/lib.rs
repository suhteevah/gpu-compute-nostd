//! # gpu-compute-nostd — Bare-metal NVIDIA GPU compute driver
//!
//! This crate provides direct GPU access without any proprietary CUDA runtime
//! or Linux kernel drivers. It talks to the hardware through MMIO registers,
//! following the architecture documented by the nouveau project and envytools.
//!
//! Designed for `#![no_std]` environments — bare-metal operating systems,
//! UEFI applications, and embedded Rust projects that need GPU compute without
//! an OS kernel or CUDA runtime.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │                    tensor.rs                              │
//! │  (TensorDescriptor, matmul, softmax, layernorm, GELU)    │
//! ├──────────────────────────────────────────────────────────┤
//! │                   compute.rs                              │
//! │  (Compute class setup, shader load, grid dispatch)        │
//! ├──────────────────────────────────────────────────────────┤
//! │                    fifo.rs                                │
//! │  (GPFIFO channels, push buffers, runlists, doorbells)     │
//! ├──────────────────────────────────────────────────────────┤
//! │                   falcon.rs                               │
//! │  (Falcon microcontroller: PMU, SEC2, GSP-RM firmware)     │
//! ├────────────────┬─────────────────────────────────────────┤
//! │  memory.rs     │              mmio.rs                     │
//! │  (VRAM, GPU    │  (NV_PMC, PFIFO, PFB, PGRAPH, etc.)     │
//! │   page tables, │                                          │
//! │   DMA mapping) │                                          │
//! ├────────────────┴─────────────────────────────────────────┤
//! │                  pci_config.rs                             │
//! │  (PCI vendor 0x10DE detect, BAR mapping, bus mastering)   │
//! ├──────────────────────────────────────────────────────────┤
//! │                   driver.rs                               │
//! │  (GpuDevice: high-level init, query, compute API)         │
//! └──────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Status
//!
//! This is scaffolding for an extraordinarily ambitious bare-metal GPU driver.
//! Real GPU initialization requires uploading signed firmware to Falcon
//! microcontrollers, constructing GPU page tables, programming the FIFO engine,
//! and speaking the compute class protocol — all of which NVIDIA keeps largely
//! undocumented. The nouveau project has reverse-engineered much of this over
//! 15+ years. We stand on their shoulders.
//!
//! ## Features
//!
//! - **PCI enumeration**: Scans for NVIDIA GPUs (vendor 0x10DE), reads BARs,
//!   enables bus mastering
//! - **MMIO register access**: Volatile read/write to GPU control registers
//!   via BAR0 mapping
//! - **GPU family detection**: Kepler through Ada Lovelace, with known device
//!   ID database
//! - **Falcon microcontroller**: Interface for PMU, SEC2, and GSP firmware
//!   upload and boot
//! - **FIFO command submission**: GPFIFO channels, push buffers, runlists,
//!   and doorbell mechanism
//! - **Compute dispatch**: Compute class binding, shader loading, grid/block
//!   configuration, kernel launch
//! - **Tensor operations**: MatMul, Softmax, LayerNorm, GELU, SiLU, RoPE
//!   for LLM inference
//! - **VRAM management**: Allocator with GPU page table entries, BAR1 CPU
//!   mapping, DMA mapping

#![no_std]

extern crate alloc;

pub mod pci_config;
pub mod mmio;
pub mod memory;
pub mod falcon;
pub mod fifo;
pub mod compute;
pub mod tensor;
pub mod driver;

pub use driver::GpuDevice;
pub use pci_config::{GpuFamily, GpuPciDevice};
pub use memory::VramAllocator;
pub use tensor::{TensorDescriptor, DType};
pub use compute::ComputeEngine;
