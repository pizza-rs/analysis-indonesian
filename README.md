# pizza-analysis-indonesian

Indonesian (Bahasa Indonesia) language analysis with stemming and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `indonesian_stem` | Token Filter | Indonesian stemmer — handles prefixes, suffixes, and circumfixes |
| `indonesian_stop` | Token Filter | Indonesian stop words filter (355 words) |
| `indonesian` | Analyzer | Full pipeline: lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "indonesian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["indonesian_stem", "indonesian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
