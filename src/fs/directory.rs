//! # Directory
//!
//! Funções para listar diretórios.

use alloc::string::String;
use alloc::vec::Vec;
use redpowder::syscall::SysResult;

use super::FileEntry;

/// Lista conteúdo de um diretório
pub fn list_directory(path: &str) -> SysResult<Vec<FileEntry>> {
    use redpowder::fs::{list_dir, FileType};

    let mut entries = Vec::new();

    // Listar diretório via syscall
    match list_dir(path) {
        Ok(read_dir) => {
            for de in read_dir {
                // Copiar nome
                let name = String::from(de.name());

                // Determinar se é diretório
                let is_directory = de.file_type() == FileType::Directory;

                // Criar entrada (tamanho 0 por enquanto)
                let entry = FileEntry::new(name, is_directory, 0);
                entries.push(entry);
            }
        }
        Err(e) => {
            redpowder::println!("[Index] Erro ao listar '{}': {:?}", path, e);
            return Err(e);
        }
    }

    // Ordenar: diretórios primeiro, depois por nome
    entries.sort_by(|a, b| {
        if a.is_directory != b.is_directory {
            // Diretórios primeiro
            b.is_directory.cmp(&a.is_directory)
        } else {
            // Ordem alfabética (case insensitive)
            a.name.to_uppercase().cmp(&b.name.to_uppercase())
        }
    });

    Ok(entries)
}
