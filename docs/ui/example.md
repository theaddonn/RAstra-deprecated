## Html:

```html
<main>
    <div>
        <label>
            This is a Label?
        </label>
        <button>
            This ia a button...
        </button>
    </div>

    // Use custom components!
    <alert>
        This is an alert!
    </alert>
</main>
```

## Rust:
```rust
use ui::builder::UiBuilder;
use ui::element::{Div, Label, Button};
use alert_elment_crate::{Alert};

#[Rastra_plugin::Entry]
fn main() {
    let builder = UiBuilder::new();
    
    let ui = builder.build(vec![
        Div::new(vec![
            Label::new(
                "This is a Label?"
            ),
            Button::new(
                "This ia a button..."
            )
        ]),
        
        // Use custom components!
        Alert::new(
            "This is an alert!"
        )
    ]);
}
```
