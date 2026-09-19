# Testausdokumentti

Automaattisilla testeillä pyritään testaamaan ohjelman toiminta mahdollisimman kattavasti. Testauksen pääpaino tulee olemaan FFT-algoritmissa sekä alipäästösuodattimessa. Lisäksi itse testauksessa käytetään muutamaa funktiota, joita testaan myös.

FFT-algoritmin testauksessa verrataan algoritmin antamia tuloksia oikeiksi todettuihin arvoihin. Arvot on pyöristetty 10 desimaalin tarkkuudelle, koska suurien liukulukujen laskutoimituksissa tapahtuu pieniä pyöristysvirheitä. FFT-algoritmin tuottama tulos taulukko on aina symmetrinen ja tätä on testattu myös.

## Testien ajaminen

Testauksessa käytetään Cargon sisäänrakennettua testaustyökalua.

Kaikkien testien ajaminen
```
cargo test
```

Yksittäisen testin ajaminen

```
cargo test -- --test <testin_nimi>
```

## Testikattavuus

Testikattavuus yksikkötestien perusteella on saatu [cargo-tarpaulin](https://docs.rs/crate/cargo-tarpaulin/latest) työkalua käyttämällä.

### Kattavuusraportti

Kattavuus: 143/185 (77.30%)

<details>
<summary>Näytä tiedostokohtainen kattavuus</summary>

| Tiedosto | Kattavuus | Prosentti | Muutos |
|------|----------|------------|--------|
| src/fft.rs | 29/29 | 100.00% | - |
| src/filter.rs | 23/49 | 46.94% | - |
| src/io.rs | 42/44 | 95.45% | - |
| src/lib.rs | 0/0 | 0.00% | - |
| src/main.rs | 0/14 | 0.00% | - |
| src/utils.rs | 49/49 | 100.00% | - |

</details>

HTML muotoinen [raportti](./testikattavuus.html) on myös saatavilla.
