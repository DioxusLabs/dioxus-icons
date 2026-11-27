<div align="center">
  <h1>dioxus-icons</h1>
  <p><strong>Dioxus components for every Lucide icon.</strong></p>
</div>

<div align="center">
  <a href="https://crates.io/crates/dioxus-icons">
    <img src="https://img.shields.io/crates/v/dioxus-icons.svg?style=flat-square" alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/dioxus-icons">
    <img src="https://img.shields.io/crates/d/dioxus-icons.svg?style=flat-square" alt="Downloads" />
  </a>
  <a href="https://docs.rs/dioxus-icons">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs" />
  </a>
  <a href="https://lucide.dev">
    <img src="https://img.shields.io/badge/Lucide-1.14.0-2da44e?style=flat-square" alt="Lucide 1.14.0" />
  </a>
</div>

---

The full Lucide set — 1,700+ icons — exposed as Dioxus components. Each icon is its own component, so the linker keeps only the ones you import.

## Quick start

```toml
[dependencies]
dioxus-icons = "0.2.0-alpha.3"
```

```rust
use dioxus::prelude::*;
use dioxus_icons::lucide::Trash;

#[component]
fn DeleteButton() -> Element {
    rsx! {
        button {
            Trash { size: 16 }
            "Delete"
        }
    }
}
```

Every icon lives under [`dioxus_icons::lucide`] and accepts the shared [`IconProps`].

## Customization

`IconProps` keeps a `size` convenience prop and extends Dioxus SVG attributes on the root SVG.

| prop / attr | default | maps to |
|---|---|---|
| `size` | `24` | SVG `width` / `height` when those attrs are not set; accepts numbers or CSS lengths |
| `stroke` | `"currentColor"` | SVG `stroke` |
| `fill` | `"none"` | SVG `fill` |
| `stroke_width` | `2` | SVG `stroke-width` |
| `stroke_linecap` | `"round"` | SVG `stroke-linecap` |
| `stroke_linejoin` | `"round"` | SVG `stroke-linejoin` |
| SVG attrs | inherited | root SVG attributes |

```rust
# use dioxus::prelude::*;
use dioxus_icons::lucide::Bell;

# let _ = rsx! {
Bell { size: 20, stroke: "red", stroke_width: 3 }
# };
```

Because `stroke` defaults to `currentColor`, icons inherit the surrounding text color — so Tailwind's `text-*` utilities (or any CSS framework) work out of the box on either the icon or its parent:

```rust
# use dioxus::prelude::*;
use dioxus_icons::lucide::{Bell, Menu};

# let _ = rsx! {
nav { class: "flex items-center gap-3 text-slate-900",
    Menu { size: 20 }
    Bell { size: 18, class: "text-amber-600" }
}
# };
```

## Dioxus compatibility

Version `0.2.0-alpha.3` targets Dioxus `0.8.0-alpha.3` (pinned exactly). For the Dioxus `0.7.x` line (from `0.7.7`), use `dioxus-icons` `0.1`. Pick your renderer features (`web`, `desktop`, `mobile`, `server`, `fullstack`) in your application crate.

## Examples

Examples live under [`crates/dioxus-icons/examples`](https://github.com/dioxuslabs/dioxus-icons/tree/main/crates/dioxus-icons/examples):

```sh
cargo run -p dioxus-icons --example basic
cargo run -p dioxus-icons --example navbar
cargo run -p dioxus-icons --example tailwind
cargo run -p dioxus-icons --example stateful_button
```

## License

Crate code is MIT (`LICENSE`). Generated icon data comes from Lucide and is covered by `LICENSE-LUCIDE` (upstream ISC plus the Feather-derived MIT notice). The published crate is `MIT AND ISC`.

[`dioxus_icons::lucide`]: https://docs.rs/dioxus-icons/latest/dioxus_icons/lucide/index.html
[`IconProps`]: https://docs.rs/dioxus-icons/latest/dioxus_icons/struct.IconProps.html
