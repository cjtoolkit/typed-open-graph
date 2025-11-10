# Typed Open Graph

The [Open Graph Protocol](https://ogp.me/) Builder that generates the structured data that you can use
with the template engine of your choice.

## Example

```rust
use typed_open_graph::{Website, OpenGraphTypeMarker};
use url::Url;

fn main() {
    let website = Website::builder(
        "https://example.com",
        Url::parse("https://example.com/image.jpg").unwrap(),
        Url::parse("https://example.com/").unwrap()
    )
        .image_type("image/jpeg")
        .build();

    // should print generated structure ready to be used in template engine
    dbg!(website);
}
```

## Installation

```sh
cargo add typed-open-graph
```