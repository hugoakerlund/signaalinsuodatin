# Käyttöohje

Asenna [Rust](https://rustup.rs/), jos se ei ole jo asennettu koneellesi.

Kloonaa repositorio:

```
git clone https://github.com/hugoakerlund/signaalinsuodatin
```

## Ohjelman kääntäminen ja riippuvuuksien asentaminen

Seuraava komento lataa kaikki tarvittavat riippuvuudet ja kääntää ne sekä projektin.

```
cargo build --release
```

Käännetty binääritiedosto on nyt hakemistossa `target/release`.

## Ohjelman käyttö

Ohjelma hyväksyy seuraavaa muotoa olevan syötteen:

```
--input <input_file> --output <output_file> --cutoff <cutoff_frequency>

```
- input_file = Ohjelmalle annettavan syötetiedoston polku. Tämä tiedosto suodatetaan.
- output_file = Ohjelmalle annettavan ulosvientitiedoston polku. Tähän tiedostoon kirjoitetaan suodatettu äänitiedosto.
- cutoff_frequency = Ylärajataajuus hertseinä. Taajuutta korkeammat taajuudet suodatetaan pois.

Esimerkki ohjelman käytöstä:

```
./target/release/signaalinsuodatin --input test_data/sample-3s.wav --output sample-3s-output.wav --cutoff 500
```

## Yksikkötestien ajaminen

Testit sijaitsevat hakemistossa `tests`.

Kaikkien testien ajaminen:

```
cargo test
```

Yksittäisen testin ajaminen:
```
cargo test -- --test <testin_nimi>
```

Dokumenttikommenttien testien ajaminen:
```
cargo test --doc
```

Voit lukea lisää Cargosta ja testin ajamisesta [täältä](https://doc.rust-lang.org/cargo/commands/cargo-test.html). Esimerkiksi testien ajaminen `--no-capture` lipulla tulostaa myös ohjelman tulosteen samalla kun testit ajetaan.

## Dokumentaation avaaminen

Seuraava komento muodostaa HTML muotoisen dokumentaation koodin dokumenttikommenttien perusteella sekä avaa sen selaimessa:

```
cargo doc --open
```

## Testikattavuusraportin luonti

[Testikattavuusraportti](testikattavuus.html) on luotu [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) työkalulla. Uuden raportin voi luoda seuraavalla komennolla:
```
cargo tarpaulin --out html
```
