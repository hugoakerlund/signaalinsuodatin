# Toteutusdokumentti

## Ohjelman yleisrakenne

Ohjelma on digitaalinen signaalin suodatin, joka poistaa ylärajataajuuttaa korkeammat taajuudet. Ohjelma ottaa komentoriviltä syötteenä polun luettavaan 16-bittiseen WAV muotoiseen äänitiedostoon, kirjoitettavan tiedoston nimen sekä ylärajataajuuden.

Komentoriviltä saadut argumentit jäsennetään, minkä jälkeen ohjelma lukee syötteenä annetun tiedoston ja tulostaa siitä tietoja käyttäjälle. Luettujen ääninäytteiden pituutta ja sämpläys taajuutta käytetään suodattimen muodostuksessa. Ohjelma laskee alipäästösuodattimen ylärajataajuuden, ääninäytteiden pituuden ja sämpläys taajuuden perusteella. Suodatin ja ääninäytteet muunnetaan FFT-algoritmilla aikatasosta taajuustasoon. Tämä jälkeen suodattimen ja ääninäytteiden konvoluutio aikatasossa saadaan kertomalla ne taajuustasossa. Taajustasossa olevat suodatetut ääninäytteet muunnetaan takaisin aikatasoon käänteisellä Fourier-muunnoksella. Kaksikanavaisten äänitiedostojen kanavat suodatetaan erikseen. Ennen kuin suodatetut ääninäytteet kirjoitetaan levylle, niiden pituus ja vaihe korjataan alkuperäiseen tilaan, joka on muuttunut Fourier-muunnoksen ja suodatuksen takia. Ohjelman ajon aikana käyttäjälle tulostetaan tietoa siitä missä vaiheessa ohjelma on.

## Saavutetut aika- ja tilavaativuudet

FFT-algoritmin aikavaativuus on O(n log n), missä n tässä tapauksessa on ääninäytteiden pituus. FFT:n saavuttama aikavaativuus perustuu siihen, että se hyödyntää yksikköjuurien erikoisominaisuuksia. Algoritmi käyttää hajoita ja hallitse strategiaa ja rekursiivisesti hajottaa n kokoisen syötteen kahteen osaan tehden sille O(n) kertolaskuoperaatiota, jolloin T(n) = 2T(n/2) + O(n). Tällöin jokainen yksikköjuuri esiintyy tasan kahdesti ja pienempien osaongelmien muoto on säilynyt samana verrattuna alkuperäiseen ongelmaan. Algoritmin aikavaativuus on huomattavasti parempi verrattuna tavallisen DFT:n aikavaativuuteen, mikä on O(n^2).

Muut ohjelman tekemät operaatiot (muunnokset kompleksiluvuista reaaliluvuiksi, ikkunafunktion luonti, signaalien konvoluutio) ovat luokkaa O(n).

Algoritmin aikavaativuus on kriittinen ohjelman käytön kannalta, koska ohjelma joutuu suorittamaan yhteensä kolme Fourier-muunnosta. Ohjelman käyttö havainnollistaa hyvin kuinka suuri ero luokilla O(n log n) ja O(n) on.

Tilavaativuuden kannalta ei ole paljon analysoitavaa, koska ohjelman vaatima muisti on suoraan verrannollinen ääninäytteiden määrään.

## Työn mahdolliset puutteet ja parannusehdotukset

Vaikka FFT-algoritmi on nimensä mukaisesti nopea, suurien syötteiden käsittely vie aikaa. Siksi FFT-algoritmin laskentaa voisi mahdollisesti rinnakkaistaa, jolloin se hyödyntäisi paremmin prosessorin resursseja ja taulukoiden käsittely olisi nopeampaa.

## Laajojen kielimallien käyttö

Tässä työssä ei ole käytetty laajoja kielimalleja.

## Lähteet

- https://ccrma.stanford.edu/~jos/sasp/Example_1_Low_Pass_Filtering.html
- https://en.wikipedia.org/wiki/Fast_Fourier_transform
- https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
- https://en.wikipedia.org/wiki/Window_function#Hamming_window
- https://en.wikipedia.org/wiki/Root_of_unity
- https://en.wikipedia.org/wiki/Sinc_filter
- https://en.wikipedia.org/wiki/Linear_phase
