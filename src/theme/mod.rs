//! # Theme
//!
//! Cores e estilos visuais do Index.

/// Paleta de cores
pub mod colors {
    /// Fundo da janela
    pub const BACKGROUND: u32 = 0xFF1E1E2E;

    /// Fundo da barra de título
    pub const TITLEBAR: u32 = 0xFF11111B;

    /// Texto da barra de título
    pub const TITLEBAR_TEXT: u32 = 0xFFCDD6F4;

    /// Fundo da breadcrumb
    pub const BREADCRUMB_BG: u32 = 0xFF181825;

    /// Texto da breadcrumb
    pub const BREADCRUMB_TEXT: u32 = 0xFF89B4FA;

    /// Fundo de item normal
    pub const ITEM_BG: u32 = 0xFF1E1E2E;

    /// Fundo de item selecionado
    pub const ITEM_SELECTED: u32 = 0xFF313244;

    /// Fundo de item hover
    pub const ITEM_HOVER: u32 = 0xFF45475A;

    /// Texto de item (nome)
    pub const ITEM_TEXT: u32 = 0xFFCDD6F4;

    /// Texto secundário (tamanho, etc)
    pub const ITEM_TEXT_SECONDARY: u32 = 0xFF6C7086;

    /// Cor de ícone de pasta
    pub const ICON_FOLDER: u32 = 0xFFF9E2AF;

    /// Cor de ícone de arquivo
    pub const ICON_FILE: u32 = 0xFF89B4FA;

    /// Cor de ícone de executável
    pub const ICON_EXECUTABLE: u32 = 0xFFA6E3A1;

    /// Borda/separador
    pub const BORDER: u32 = 0xFF45475A;

    /// Botão de voltar
    pub const BACK_BUTTON: u32 = 0xFF89B4FA;
}

/// Dimensões e layout
pub mod layout {
    /// Altura da barra de título
    pub const TITLEBAR_HEIGHT: u32 = 32;

    /// Altura da breadcrumb
    pub const BREADCRUMB_HEIGHT: u32 = 32;

    /// Altura de cada item
    pub const ITEM_HEIGHT: u32 = 48;

    /// Largura do ícone
    pub const ICON_SIZE: u32 = 32;

    /// Padding horizontal
    pub const PADDING_H: u32 = 16;

    /// Padding vertical
    pub const PADDING_V: u32 = 8;

    /// Espaço entre ícone e texto
    pub const ICON_TEXT_GAP: u32 = 12;
}
