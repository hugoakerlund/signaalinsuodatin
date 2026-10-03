# Viikkoraportti 5

Tällä viikolla olen tehnyt ensimmäisen vertaisarvion. Vertaisarvioitavassa projektissa oli sama teema kuin omassani ja sain ideoita oman aiheeni testausta varten. Vertaisarvion tekeminen auttoi minua myös löytämään omassa koodissani olevan virheen. Kaksikanavaisten äänitiedostojen kanavat suodatetaan nyt erikseen kuten kuuluukin. Lisäksi olen kirjoittanut käyttöohjeen ja täydentänyt testausraporttia. Olen lisännyt repositorioon lyhyitä videoita, jotka havainnollistavat suodatettuja äänitiedostoja graafisesti.

Olen panostanut suodatettujen ääninäytteiden oikeellisuuden testaamiseen. Suodattimen toimintaa testataan vertaamalla ylärajataajuuden alapuolella sekä yläpuolella olevien taajuuksien voimakkuuksia ja varmistamalla että ylärajataajuutta korkeammat taajuudet ovat vaimentuneet hyväksyttävälle tasolle. Digitaaliset suodattimet eivät ole koskaan täydellisiä ja siksi testeissä sallitaan, että muutama prosentti voimakkuuksista esiintyy ylärajataajuuden yläpuolella. Lisäksi testeissä verrataan kahta synteettisesti tuotettua signaalia, joista toisessa esiintyy ylärajataajuutta korkeampi taajuus. Signaali suodatetaan ja sitä verrataan signaaliin, jossa ei esiintynyt ylärajataajuutta korkeampaa taajuutta.

Olen oppinut FFT:n taajuuskorien ja niitä vastaavien voimakkuuksien laskemisesta. Lisäksi olen oppinut oman projektinin testaamiseen liittyvistä tekniikoista.

Seuraavaksi keskityn ohjelman dokumentaation ja testien viimeistelyyn sekä hiontaan.

Arvioitu käytetty tuntimäärä tällä viikolla: 17h
