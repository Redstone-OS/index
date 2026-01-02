//! # Explorer
//!
//! Lógica principal do explorador de arquivos.

use alloc::string::String;
use alloc::vec::Vec;
use redpowder::println;
use redpowder::syscall::SysResult;
use redpowder::window::Window;

use crate::fs::{list_directory, FileEntry};
use crate::theme::colors;
use crate::ui::{draw_breadcrumb, draw_file_grid, draw_titlebar};

/// Estado do explorador
pub struct Explorer {
    /// Janela principal
    window: Window,
    /// Caminho atual
    current_path: String,
    /// Entradas do diretório atual
    entries: Vec<FileEntry>,
    /// Largura da janela
    width: u32,
    /// Altura da janela
    height: u32,
    /// Item selecionado (índice)
    selected: Option<usize>,
    /// Offset de scroll
    scroll_offset: usize,
    /// Precisa redesenhar
    dirty: bool,
}

impl Explorer {
    /// Cria novo explorador
    pub fn new() -> SysResult<Self> {
        let width = 800;
        let height = 600;

        // Criar janela
        let window = Window::create(100, 50, width, height, "Index")?;

        println!("[Index] Janela criada: {}x{}", width, height);

        let mut explorer = Self {
            window,
            current_path: String::from("/"),
            entries: Vec::new(),
            width,
            height,
            selected: None,
            scroll_offset: 0,
            dirty: true,
        };

        // Carregar diretório inicial
        explorer.load_directory();

        Ok(explorer)
    }

    /// Carrega o diretório atual
    fn load_directory(&mut self) {
        println!("[Index] Carregando: {}", self.current_path);

        match list_directory(&self.current_path) {
            Ok(entries) => {
                self.entries = entries;
                self.selected = None;
                self.scroll_offset = 0;
                self.dirty = true;
                println!("[Index] {} entradas encontradas", self.entries.len());
            }
            Err(e) => {
                println!("[Index] Erro ao listar: {:?}", e);
                self.entries.clear();
            }
        }
    }

    /// Navega para um diretório
    fn navigate_to(&mut self, path: &str) {
        self.current_path = String::from(path);
        self.load_directory();
    }

    /// Navega para o diretório pai
    fn navigate_up(&mut self) {
        if self.current_path == "/" {
            return;
        }

        // Encontrar última /
        if let Some(pos) = self.current_path.rfind('/') {
            if pos == 0 {
                self.current_path = String::from("/");
            } else {
                self.current_path = String::from(&self.current_path[..pos]);
            }
            self.load_directory();
        }
    }

    /// Entra no item selecionado
    fn enter_selected(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.entries.len() {
                let entry = &self.entries[idx];

                if entry.is_directory {
                    // Navegar para o diretório
                    let new_path = if self.current_path == "/" {
                        alloc::format!("/{}", entry.name)
                    } else {
                        alloc::format!("{}/{}", self.current_path, entry.name)
                    };
                    self.navigate_to(&new_path);
                } else {
                    // TODO: Abrir arquivo ou executar
                    println!("[Index] Arquivo: {}", entry.name);
                }
            }
        }
    }

    /// Redesenha a interface
    fn redraw(&mut self) {
        let buf = self.window.buffer();
        let w = self.width;
        let h = self.height;

        // Limpar fundo
        for pixel in buf.iter_mut() {
            *pixel = colors::BACKGROUND;
        }

        // Desenhar barra de título
        draw_titlebar(buf, w, h, "Index - File Explorer");

        // Desenhar breadcrumb/caminho
        draw_breadcrumb(buf, w, h, &self.current_path);

        // Desenhar grid de arquivos
        draw_file_grid(buf, w, h, &self.entries, self.selected, self.scroll_offset);

        // Apresentar
        let _ = self.window.present();
    }

    /// Loop principal
    pub fn run(&mut self) -> ! {
        println!("[Index] Entrando no loop principal...");

        loop {
            // Processar eventos
            self.process_events();

            // Redesenhar se necessário
            if self.dirty {
                self.redraw();
                self.dirty = false;
            }

            // Throttle
            let _ = redpowder::time::sleep(16);
        }
    }

    /// Processa eventos de input
    fn process_events(&mut self) {
        use redpowder::event::{event_type, Event};

        // Coletar eventos primeiro para evitar borrow múltiplo
        let events: alloc::vec::Vec<_> = self.window.poll_events().collect();

        for event in events {
            match event {
                Event::Input(input) => match input.event_type {
                    event_type::KEY_DOWN => {
                        let key = input.param1;
                        self.handle_key(key);
                    }
                    event_type::MOUSE_DOWN => {
                        let x = input.param1 as i32;
                        let y = (input.param2 >> 16) as i32;
                        self.handle_click(x, y);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /// Trata tecla pressionada
    fn handle_key(&mut self, key: u32) {
        match key {
            // Backspace - voltar
            14 => {
                self.navigate_up();
            }
            // Enter - entrar
            28 => {
                self.enter_selected();
            }
            // Seta para cima
            72 => {
                if let Some(idx) = self.selected {
                    if idx > 0 {
                        self.selected = Some(idx - 1);
                        self.dirty = true;
                    }
                } else if !self.entries.is_empty() {
                    self.selected = Some(0);
                    self.dirty = true;
                }
            }
            // Seta para baixo
            80 => {
                if let Some(idx) = self.selected {
                    if idx + 1 < self.entries.len() {
                        self.selected = Some(idx + 1);
                        self.dirty = true;
                    }
                } else if !self.entries.is_empty() {
                    self.selected = Some(0);
                    self.dirty = true;
                }
            }
            // Escape - fechar
            1 => {
                println!("[Index] Fechando...");
                redpowder::process::exit(0);
            }
            _ => {}
        }
    }

    /// Trata clique do mouse
    fn handle_click(&mut self, x: i32, y: i32) {
        // Área do grid começa após titlebar (32px) e breadcrumb (32px)
        let content_y = 64;
        let item_height = 48;

        if y < content_y {
            // Clique na breadcrumb - voltar
            if y >= 32 && y < 64 && x < 40 {
                self.navigate_up();
            }
            return;
        }

        // Calcular item clicado
        let relative_y = (y - content_y) as usize;
        let idx = self.scroll_offset + relative_y / item_height as usize;

        if idx < self.entries.len() {
            if self.selected == Some(idx) {
                // Duplo clique - entrar
                self.enter_selected();
            } else {
                self.selected = Some(idx);
                self.dirty = true;
            }
        }
    }
}
