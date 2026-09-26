# Viikkoraportti 4

Tällä viikolla olen refaktoroinut koodia ja parantanut sen dokumentaatiota ja luettavuutta. Olen myös aloittanut toteutusdokumentin kirjoittamisen. Olen parhaani mukaan luonut testejä suodatinta varten ja täydentänyt testausraporttia. Olen myös lisännyt testidataa suodatinta varten. Lisäksi olen hyödyntänyt Rustin dokumenttikommentteja. Nyt `cargo doc` luo HTML muotoisen dokumentaation hakemistoon`target/doc` ja `cargo doc --open` avaa sen selaimessa. Lisäksi `cargo test` ajaa myös dokumenttikommenttien testit.

Ohjelma toimii kuten viime viikollakin, mutta koodin luettavuus, rakenne ja dokumentaatio ovat parantuneet. Olen refaktoroinut suodattimen ja FFT-algoritmin omiksi luokiksi sekä pilkkonut ja jäsennellyt koodia. Lisäksi ohjelman testaus on kattavampaa.

Olen lukenut hieman lisää aiheeseen liittyvää materiaalia syventääkseni ymmärrystäni ja voidakseni dokumentoida koodia paremmin. Esimerkiksi lineaarinen vaihe suodattimen ominaisuutena oli minulle uutta. Toteutetulla suodattimella on juurikin tämä ominaisuus ja sen takia suodatetun signaalin vaihe täytyy korjata, koska se on viivästynyt suodattimen käytön jälkeen.

Seuraavaksi aloitan käyttöohjeen kirjoittamisen ja täydennän toteutusdokumenttia sekä testausraporttia. Teen myös ensimmäisen vertaisarvion seuraavalla viikolla.

Arvioitu käytetty tuntimäärä tällä viikolla: 10h
