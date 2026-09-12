# Viikkoraportti 2

Tällä viikolla olen toteuttanut rekursiivisen FFT-algoritmin sekä sen käänteisfunktion. Olen samalla myös keskittynyt koodin testaamiseen, parantanut testikattavuutta sekä valinnut tavan luoda testiraportteja. Olen myös alustavasti dokumentoinut koodia kommenteilla.

Ohjelman ydin algoritmi on nyt toteutettu ja muunnos aikatasosta taajuustasoon ja takaisin on nyt mahdollista. Ohjelma pystyy lukemaan ja kirjoitamaan äänitiedoston komentoriviltä annettujen argumenttien perusteella, mutta ohjelma ei vielä suodata signaalia. Projektin testikattavuus on hyvä ja se on seurattavissa. Testiraportin voi generoida komennolla `cargo tarpaulin --out html`, jos tarpaulin on asennettu.

Olen oppinut paljon FFT-algoritmin toteutuksesta ja testauksesta. Ymmärrän algoritmin toimintaperiaatteen ja miten se eroaa tavallisesta DFT-algoritmista. Vaikka algoritmi toimii oikein, sen testaaminen voi olla joskus hieman haastavaa, koska suurilla liukuluvuilla tapahtuvissa laskutoimituksissa syntyy pieniä pyöristysvirheitä. En tiennyt aikaisemmin, että kyseinen algoritmi toimii vain syötteille joiden pituus on 2^n ja että lyhyemmät syötteet täytyy täyttää nollilla tähän pituuteen asti tai käyttää jotakin toista Fourier-muunnos algoritmia. Pythonin numpy ilmeisesti käyttää jotakin toista muunnelmaa FFT-algoritmista, koska se voi laskea muunnoksen myös syötteille, joiden pituus ei ole 2^n.

Seuraavaksi alan miettiä alipäästösuodattimen toteutusta ja käsittelemään luettua äänitiedostoa FFT-algoritmilla, jotta voin poistaa korkeat taajuudet alipäästösuodattimella. Epäselviä asioita ei tällä hetkellä ole. Aloitan myös testausdokumentin kirjoittamisen.

Arvioitu käytetty tuntimäärä tällä viikolla: 16h
