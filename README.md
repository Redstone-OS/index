# Index - RedstoneOS File Explorer

Um explorador de arquivos simples e modular para o RedstoneOS.

## Características

- **Navegação de diretórios**: Navegar por `/`, `/system`, `/apps`, etc.
- **Interface visual**: Barra de título, breadcrumb e lista de arquivos
- **Tema escuro**: Inspirado no Catppuccin Mocha
- **Controles**: Teclado e mouse

## Estrutura do Projeto

```
index/
├── src/
│   ├── main.rs           # Ponto de entrada
│   ├── lib.rs            # Módulo raiz
│   ├── app/              # Lógica da aplicação
│   │   ├── mod.rs
│   │   └── explorer.rs   # Estado e lógica do explorador
│   ├── fs/               # Operações de filesystem
│   │   ├── mod.rs
│   │   ├── directory.rs  # Listagem de diretórios
│   │   └── entry.rs      # Estrutura de entrada de arquivo
│   ├── ui/               # Interface gráfica
│   │   ├── mod.rs
│   │   ├── components.rs # Componentes visuais
│   │   └── drawing.rs    # Funções de desenho
│   └── theme/            # Tema visual
│       └── mod.rs        # Cores e dimensões
├── .cargo/
│   └── config.toml       # Configuração de build
├── Cargo.toml
├── linker.ld
└── rust-toolchain.toml
```

## Controles

| Tecla/Ação     | Função                    |
|----------------|---------------------------|
| ↑ / ↓          | Navegar entre itens       |
| Enter          | Entrar em diretório       |
| Backspace      | Voltar para diretório pai |
| Escape         | Fechar aplicação          |
| Clique simples | Selecionar item           |
| Clique duplo   | Entrar em diretório       |

## Build

```bash
cargo build --release
```

## Roadmap

- [ ] Suporte a execução de binários
- [ ] Preview de conteúdo de arquivos
- [ ] Scroll com mouse wheel
- [ ] Operações de arquivo (copiar, mover, deletar)
- [ ] Favoritos/Bookmarks
- [ ] Busca de arquivos

## Licença

MIT License - RedstoneOS Team
