//! # Index - Ponto de Entrada
//!
//! Explorador de arquivos para RedstoneOS.

#![no_std]
#![no_main]

extern crate alloc;

use index::app::Explorer;
use redpowder::println;

/// Alocador global
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    main();
}

fn main() -> ! {
    println!("[Index] Iniciando v0.1.0...");

    // Criar e executar o explorador
    match Explorer::new() {
        Ok(mut explorer) => {
            println!("[Index] Explorador iniciado!");
            explorer.run();
        }
        Err(e) => {
            println!("[Index] Erro ao iniciar: {:?}", e);
            loop {
                let _ = redpowder::time::sleep(1000);
            }
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[Index] PANIC: {:?}", info);
    loop {
        let _ = redpowder::time::sleep(1000);
    }
}
