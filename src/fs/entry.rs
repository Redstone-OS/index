//! # FileEntry
//!
//! Representação de uma entrada de arquivo/diretório.

use alloc::string::String;

/// Tipo de entrada
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    /// Arquivo regular
    File,
    /// Diretório
    Directory,
    /// Executável
    Executable,
    /// Desconhecido
    Unknown,
}

/// Entrada de arquivo/diretório
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Nome do arquivo/diretório
    pub name: String,
    /// É diretório?
    pub is_directory: bool,
    /// Tamanho em bytes (0 para diretórios)
    pub size: u64,
    /// Tipo da entrada
    pub entry_type: EntryType,
}

impl FileEntry {
    /// Cria nova entrada
    pub fn new(name: String, is_directory: bool, size: u64) -> Self {
        let entry_type = if is_directory {
            EntryType::Directory
        } else if name.ends_with(".exe") || !name.contains('.') {
            // Heurística simples: sem extensão = executável
            EntryType::Executable
        } else {
            EntryType::File
        };

        Self {
            name,
            is_directory,
            size,
            entry_type,
        }
    }

    /// Retorna ícone para o tipo
    pub fn icon(&self) -> char {
        match self.entry_type {
            EntryType::Directory => '📁',
            EntryType::Executable => '⚙',
            EntryType::File => '📄',
            EntryType::Unknown => '❓',
        }
    }
}
