//! # Components
//!
//! Componentes visuais do Index.

use alloc::vec::Vec;

use crate::fs::FileEntry;
use crate::theme::{colors, layout};

use super::drawing::{draw_text, fill_rect, hline};

/// Desenha a barra de título
pub fn draw_titlebar(buf: &mut [u32], w: u32, _h: u32, title: &str) {
    // Fundo
    fill_rect(buf, w, 0, 0, w, layout::TITLEBAR_HEIGHT, colors::TITLEBAR);

    // Título centralizado
    let text_x = (w / 2).saturating_sub((title.len() as u32 * 8) / 2);
    draw_text(buf, w, text_x, 12, title, colors::TITLEBAR_TEXT);

    // Linha separadora
    hline(buf, w, 0, layout::TITLEBAR_HEIGHT - 1, w, colors::BORDER);
}

/// Desenha a breadcrumb (caminho atual)
pub fn draw_breadcrumb(buf: &mut [u32], w: u32, _h: u32, path: &str) {
    let y = layout::TITLEBAR_HEIGHT;

    // Fundo
    fill_rect(
        buf,
        w,
        0,
        y,
        w,
        layout::BREADCRUMB_HEIGHT,
        colors::BREADCRUMB_BG,
    );

    // Botão de voltar
    draw_text(buf, w, 8, y + 12, "<", colors::BACK_BUTTON);

    // Caminho
    let path_x = 32;
    draw_text(buf, w, path_x, y + 12, path, colors::BREADCRUMB_TEXT);

    // Linha separadora
    hline(
        buf,
        w,
        0,
        y + layout::BREADCRUMB_HEIGHT - 1,
        w,
        colors::BORDER,
    );
}

/// Desenha o grid de arquivos
pub fn draw_file_grid(
    buf: &mut [u32],
    w: u32,
    h: u32,
    entries: &Vec<FileEntry>,
    selected: Option<usize>,
    scroll_offset: usize,
) {
    let start_y = layout::TITLEBAR_HEIGHT + layout::BREADCRUMB_HEIGHT;
    let item_h = layout::ITEM_HEIGHT;
    let max_visible = ((h - start_y) / item_h) as usize;

    for (i, entry) in entries
        .iter()
        .skip(scroll_offset)
        .take(max_visible)
        .enumerate()
    {
        let y = start_y + (i as u32 * item_h);
        let is_selected = selected == Some(scroll_offset + i);

        draw_file_item(buf, w, y, entry, is_selected);
    }

    // Mensagem se vazio
    if entries.is_empty() {
        draw_text(
            buf,
            w,
            16,
            start_y + 16,
            "Diretorio vazio",
            colors::ITEM_TEXT_SECONDARY,
        );
    }
}

/// Desenha um item de arquivo
fn draw_file_item(buf: &mut [u32], w: u32, y: u32, entry: &FileEntry, selected: bool) {
    let item_h = layout::ITEM_HEIGHT;

    // Fundo
    let bg_color = if selected {
        colors::ITEM_SELECTED
    } else {
        colors::ITEM_BG
    };
    fill_rect(buf, w, 0, y, w, item_h, bg_color);

    // Ícone (representado como quadrado colorido)
    let icon_color = if entry.is_directory {
        colors::ICON_FOLDER
    } else {
        colors::ICON_FILE
    };
    let icon_x = layout::PADDING_H;
    let icon_y = y + (item_h - 24) / 2;
    fill_rect(buf, w, icon_x, icon_y, 24, 24, icon_color);

    // Símbolo no ícone
    let symbol = if entry.is_directory { "D" } else { "F" };
    draw_text(buf, w, icon_x + 8, icon_y + 8, symbol, colors::TITLEBAR);

    // Nome
    let text_x = icon_x + 24 + layout::ICON_TEXT_GAP;
    let text_y = y + (item_h - 8) / 2;
    draw_text(buf, w, text_x, text_y, &entry.name, colors::ITEM_TEXT);

    // Linha separadora
    hline(buf, w, 0, y + item_h - 1, w, colors::BORDER);
}
