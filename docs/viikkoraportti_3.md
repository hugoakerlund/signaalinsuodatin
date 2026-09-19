# Viikkoraportti 3

Tällä viikolla olen toteuttanut alipäästösuodattimen. Suodattimen toteutus perustuu FFT konvoluutioon ja sen toteutus pohjautuu pitkälti Stanfordin materiaaliin (https://ccrma.stanford.edu/~jos/sasp/Example_1_Low_Pass_Filtering.html). Alipäästösuodatin selvästi toimii ja sen avulla saadaan vaimennettua korkea taajuudet samalla kun matalat taajuudet säilyvät. Lisäksi olen lisännyt yksikkötestejä ja aloittanut testausraportin kirjoittamisen.

Ohjelma toimii ja se pystyy suodattaan korkeat taajuudet pois. Ohjelma käyttää FFT-algoritmia yhteensä kolme kertaa, joten pitkien ääninäytteyden käsittely vie melko paljon aikaa, vaikka algoritmi toimiikin ajassa O(n log n). Etenkin kaksikanavaisten äänitiedostojen suodattaminen vie aikaa, koska ääninäytteiden määrä on kaksinkertainen.

Olen oppinut erityisesti digitaalisista suodattimista ja ikkunafunktiosta. Etenkin FIR-suodattimet ja signaalien konvoluutio aika- ja taajustasossa oli minulle uutta. Opin myös sinc- ja ikkunafunktioiden merkityksestä signaalinkäsittelyssä.

Seuraavaksi alan jäsentelemään koodia selkeämmäksi. Aion myös parantaa koodin dokumentaatiota ja testikattavuutta. Yritän keksiä miten alipäästösuodattimen toimivuutta voidaan testata. Aloitan myös totetusdokumentin kirjoittamisen.

Arvioitu käytetty tuntimäärä tällä viikolla: 15h
