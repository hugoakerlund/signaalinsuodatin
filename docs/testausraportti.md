# Testausdokumentti

Automaattisilla testeillä pyritään testaamaan ohjelman toiminta mahdollisimman kattavasti. Testauksen pääpaino on FFT-algoritmissa sekä alipäästösuodattimessa. Lisäksi itse testauksessa käytetään muutamaa funktiota, joita testaan myös.

## FFT-algoritmin testaus

FFT-algoritmin testauksessa verrataan algoritmin antamia tuloksia oikeiksi todettuihin arvoihin, jotka on laskettu Pythonin numpy kirjastolla. Arvot on pyöristetty 10 desimaalin tarkkuudelle, koska suurien liukulukujen laskutoimituksissa tapahtuu pieniä pyöristysvirheitä. FFT-algoritmin tuottama tulos taulukko on aina symmetrinen ja tätä on testattu myös.

Algoritmi hyödyntää myös muutamaa pienempää funktiota, joiden testaus on suoraviivaista. Yksikköjuuria laskevan funktion odotetut arvot on laskettu laskimella ja näitä ollaan verrattu funktion tuottamiin tuloksiin.

## Suodattimen testaus

Hamming-ikkunan laskevan funktion ja sinc -funktion odotetut arvot on laskettu laskimella ja näitä ollaan verrattu funktioiden tuottamiin tuloksiin.

Alipäästösuodattimella suodatetut ääniraidat sisältävät vähemmän äänidataa. Tätä ollaan testattu varmistamalla, että suodatettujen ääninäytteiden summa on pienempi kuin alkuperäisten ääninäytteiden.

Suodattimelle tarkoitettu testidata on ladattu [täältä](https://samplelib.com/sample-wav.html).

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

Dokumenttikommentti testien ajaminen

```
cargo test --doc
```

## Testikattavuus

Testikattavuus yksikkötestien perusteella on saatu [cargo-tarpaulin](https://docs.rs/crate/cargo-tarpaulin/latest) työkalua käyttämällä.

### Kattavuusraportti

Kattavuus: 183/199 (91.96%)

<details>
<summary>Näytä tiedostokohtainen kattavuus</summary>

| Tiedosto | Kattavuus | Prosentti | Muutos |
|------|----------|------------|--------|
| src/fft.rs | 36/36 | 100.00% | - |
| src/filter.rs | 85/85 | 100.00% | - |
| src/io.rs | 42/44 | 95.45% | - |
| src/lib.rs | 0/0 | 0.00% | - |
| src/main.rs | 0/14 | 0.00% | - |
| src/utils.rs | 20/20 | 100.00% | - |

</details>

HTML muotoinen [raportti](./testikattavuus.html) on myös saatavilla.
