//! # Index - RedstoneOS File Explorer
//!
//! Um explorador de arquivos simples e modular para o RedstoneOS.
//!
//! ## Módulos
//!
//! - `app` - Lógica principal da aplicação
//! - `ui` - Componentes de interface gráfica
//! - `fs` - Operações de filesystem
//! - `theme` - Cores e estilos visuais

#![no_std]
#![no_main]

extern crate alloc;

pub mod app;
pub mod fs;
pub mod theme;
pub mod ui;
