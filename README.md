<div align="center">

# 🇮🇩 pizza-analysis-indonesian

**Indonesian text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--indonesian-blue)](https://github.com/pizza-rs/analysis-indonesian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Indonesian (Bahasa Indonesia) language analysis with stemming and stop words.
Handles Indonesian affixation (prefixes me-, ber-, di-, ke-, per- and suffixes -kan, -an, -i).

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `indonesian_stem` | Indonesian stemmer (affix stripping) |
| TokenFilter | `indonesian_stop` | Indonesian stop words (355 entries) |
| Analyzer | `indonesian` | Full pipeline: lowercase → stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_indonesian::register_all(&mut factory);

let analyzer = factory.get_analyzer("indonesian").unwrap();
// "mempermasalahkan" → "masalah"
```

## Installation

```toml
[dependencies]
pizza-analysis-indonesian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["indonesian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
