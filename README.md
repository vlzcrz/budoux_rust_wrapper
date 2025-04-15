# BudouX Rust Wrapper

Una implementación en Rust de [BudouX](https://github.com/google/budoux), una herramienta para organizar saltos de línea en texto japonés.

## Características

- Segmentación de texto japonés en frases semánticas
- API simple y fácil de usar
- Implementación eficiente en Rust
- Incluye el modelo japonés preentrenado

## Instalación

Agrega esta dependencia a tu archivo `Cargo.toml`:

```toml
[dependencies]
budoux_rust_wrapper = "0.1.0"
```

O directamente desde el repositorio Git:

```toml
[dependencies]
budoux_rust_wrapper = { git = "https://github.com/tu-usuario/budoux_rust_wrapper", branch = "main" }
```

## Uso

### Como biblioteca

```rust
use budoux_rust_wrapper::load_default_japanese_parser;

fn main() {
    // Cargar el parser japonés por defecto
    let parser = load_default_japanese_parser();

    // Segmentar texto
    let result = parser.parse("今日は天気です。");

    // Imprimir resultado
    println!("{:?}", result); // ["今日は", "天気です。"]
}
```

### Como herramienta de línea de comandos

Para usar la interfaz de línea de comandos, compila con la característica `cli`:

```bash
cargo install budoux_rust_wrapper --features cli
```

Luego puedes usarlo así:

```bash
# Formato de texto (por defecto)
budoux "今日は天気です。"
今日は
天気です。

# Formato JSON
budoux --format json "今日は天気です。"
[
  "今日は",
  "天気です。"
]
```

## Licencia

Este proyecto está licenciado bajo la licencia Apache 2.0 - ver el archivo LICENSE para más detalles.
