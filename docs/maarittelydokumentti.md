# Määrittelydokumentti

Tämä dokumentti määrittelee Helsingin yliopiston tietojenkäsittelytieteen kandiohjelman syksyn 2026 ensimmäisen periodin aineopintojen harjoitustyön kurssilla Algoritmit ja tekoäly.

## Aihe

Työn aiheena on digitaalisen äänisignaalin taajuussuodatus Fourier-muunnoksen avulla.

Ohjelma lukee WAV tiedostomuotoisen äänitiedoston, muuntaa sen aikatasosta taajuustasoon Fast Fourier Transform -algoritmilla eli FFT:llä, poistaa käyttäjän määrittelemän ylärajataajuuden ylittävät taajuuskomponentit ja muuntaa signaalin takaisin aikatasoon käänteisellä Fourier-muunnoksella. Lopputulos kirjoitetaan uuteen äänitiedostoon.

Ohjelma on terminaalissa ajettava komentorivisovellus.

## Ratkaistava ongelma

Työn tavoitteena on toteuttaa ohjelma, jolla äänisignaalista voidaan poistaa liian korkeat taajuudet. Tätä kutsutaan alipäästösuodatukseksi.

Käyttäjä antaa ohjelmalle äänitiedoston ja taajuuden, jota suuremmat taajuuskomponentit poistetaan. Ohjelma säilyttää ylärajataajuutta pienemmät taajuuskomponentit ja vaimentaa ylärajataajuutta suuremmat komponentit.

## Syötteet ja tulosteet

Ohjelma saa komentoriviltä seuraavat syötteet:
- syötteenä käytettävän äänitiedoston polun
- tulostiedoston polun
- ylärajataajuuden hertseinä

## Algoritmit

Työssä toteutetaan rekursiivinen Cooley–Tukey FFT-algoritmi, jonka aikavaativuus on O(n log n). Algoritmi jakaa n näytteen pituisen signaalin parillisiin ja parittomiin näytteisiin, laskee näille pienemmät Fourier-muunnokset ja yhdistää tulokset.

## Ohjelmointikielet

Työ toteutetaan Rust-ohjelmointikielellä.

Pystyn tarvittaessa vertaisarvioimaan Pythonilla, C:llä ja C++:lla tehtyjä projekteja.

## Työn ydin

Työn ydin on Cooley–Tukey FFT-algoritmin toteuttaminen ja sen käyttäminen äänisignaalin taajuussuodatuksessa. Äänitiedostojen lukeminen, komentoriviparametrien käsittely ja tulostiedoston kirjoittaminen ovat tarpeellisia toimivan sovelluksen tekemiseksi, mutta eivät työn pääpaino.

## Lähteet

- https://en.wikipedia.org/wiki/Fast_Fourier_transform
- https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
- https://en.wikipedia.org/wiki/Window_function
- https://en.wikipedia.org/wiki/Hann_function
- https://ccrma.stanford.edu/~jos/sasp/Example_1_Low_Pass_Filtering.html
- https://ccrma.stanford.edu/~jos/mdft/Fast_Fourier_Transform_FFT.html

## Dokumentaatio

Harjoitustyön ohjelmakoodi, muuttujat ja funktiot kirjoitetaan englanniksi. Koodin dokumentointi (kommentit) ja muu dokumentaatio (määrittely-, toteutus- ja testausdokumentti sekä käyttöohje ja viikkoraportit) kirjoitetaan suomeksi.
