# Testausdokumentti

Automaattisilla testeillä pyritään testaamaan ohjelman toiminta mahdollisimman kattavasti. Testauksen pääpaino on FFT-algoritmissa sekä alipäästösuodattimessa. Lisäksi itse testauksessa käytetään muutamaa funktiota, joita testaan myös.

## FFT-algoritmin testaus

FFT-algoritmin testauksessa verrataan algoritmin antamia tuloksia oikeiksi todettuihin arvoihin, jotka on laskettu Pythonin numpy kirjastolla. Arvot on pyöristetty 10 desimaalin tarkkuudelle, koska suurien liukulukujen laskutoimituksissa tapahtuu pieniä pyöristysvirheitä.

FFT-algoritmin tuottamalla taulukolla on tiettyjä ominaisuuksia. FFT-algoritmin tuottama tulos taulukko on aina symmetrinen. Sen oikea puoli on vasemman puolen kompleksikonjungaatti ja tätä ollaan testattu. Lisäksi algoritmin tuottaman taulukon ensimmäinen alkio on aina sen syötteenä olleen taulukon alkioiden summa.

Algoritmi hyödyntää myös muutamaa pienempää funktiota, joiden testaus on suoraviivaista. Yksikköjuuria laskevan funktion odotetut arvot on laskettu laskimella ja näitä ollaan verrattu funktion tuottamiin tuloksiin.

## Suodattimen testaus
Suodatettujen tiedostojen taajuuksien voimakkuuksia ylärajataajuuden alapuolella ja yläpuolella ollaan verrattu. Testeissä ollaan varmistettu, että ylärajataajuuden yläpuolella olevien taajuuksien voimakkuus on enintään vain muutama prosentti kaikkien taajuuksien voimakkuuksista.

Suodatinta ollaan testattu myös luomalla synteettisiä signaaleja. Ollaan testattu, että ylärajataajuutta suuremmat signaalit suodattuvat pois ja alemmat säilyvät mahdollisimman hyvin.

Alipäästösuodattimella suodatetut ääniraidat sisältävät vähemmän äänidataa. Tätä ollaan testattu varmistamalla, että suodatettujen ääninäytteiden summa on pienempi kuin alkuperäisten ääninäytteiden.

Hamming-ikkunan laskevan funktion ja sinc -funktion odotetut arvot on laskettu laskimella ja näitä ollaan verrattu funktioiden tuottamiin tuloksiin.


Suodattimelle tarkoitettu testidata on ladattu [täältä](https://samplelib.com/sample-wav.html).

## Suodatetun tiedoston havainnollistus

Repositoriossa on lyhyitä [videoita](../videos), jotka havainnollistavat tiedoston `sample-3s.wav` suodatusta eri taajuuksilla. Tiedoston nimi kertoo millä ylärajataajudella tiedoston on suodatettu. Kuvaajan punaiset merkit kertovat kuinka voimakkaana taajuus on esiintynyt ääninäytteessä. Videot on luotu [tämän](https://wutools.com/audio/spectrum-analyzer) työkalun avulla.

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

Kattavuus: 264/277 (95.31% +0.92%)

<details>
<summary>Näytä tiedostokohtainen kattavuus</summary>

| Tiedosto | Kattavuus | Prosentti | Muutos |
|------|----------|------------|--------|
| src/fft.rs | 48/48 | 100.00% | - |
| src/filter.rs | 82/82 | 100.00% | - |
| src/io.rs | 41/43 | 95.35% | +0.23% |
| src/lib.rs | 0/0 | 0.00% | - |
| src/main.rs | 0/11 | 0.00% | - |
| src/utils.rs | 51/51 | 100.00% | - |
| src/wav_filter.rs | 42/42 | 100.00% | - |

</details>

HTML muotoinen [raportti](./testikattavuus.html) on myös saatavilla.
