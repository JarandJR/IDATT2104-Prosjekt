# IDATT2104-Prosjekt
Frivillighetsprosjektet i Nettverksprogrammering

## Introduksjon 
I emnet IDATT2104 - Nettverksprogrammering, ble det presentert en frivillig oppgave med fokus på mobile wireless mesh-nettverk. Studentene Jarand Romestrand og Vetle Nordang valgte å ta denne utfordringen, og utviklet en simulering som illustrerer hvordan en gruppe droner kan kommunisere med hverandre. Målet var å simulere søk etter savnede personer i et bestemt område, for så å returnere til avsender med status på søket. Dette prosjektet krevde en betydelig innsats, med over 35 timer arbeid fra  hver student. Simuleringen har fått navnet "SkyNet: Autonomous Drone Search And Rescue Simulation".

## Oppgavebeskrivelse

### Motivasjon
Ulykker forekommer på plasser med dårlig eller ingen kommunikasjons kanaler. Dette kan være alt fra et snøras til at noen har forsvunnet i på en skogstur eller til fjels når en er på hytta. Når dette hender vil et redningsmannskap bli sent ut på leting. Et godt hjelpeverktøy her vil være et mobilt tråløst mesh nettverk laget av droner. Dette vil hjelpe redningsmaskapet med å dekke større områder, dronene vil kunne hjelpe med letingen ved hjelp av kameraene sine som vil se etter den savnede, ett effektiv kamera ville her vært et varmesøkende kamera. 

### Problemstilling
Problem stillingen går ut på å lage en simulering av droner som flyr over et geografisk område for å hjelpe et søkemannskap med selve søket gjennom varmesøkende kameraer. Fokuset på simuleringen vil være å kunne legge til en drone til allerede flygende droner. Tanken er å lage en eller flere terminaler som skal representere en drone hver. Terminalen vil skrive ut informasjonen som går gjennom datapakkene som blir sent gjennom UDP. Når en drone finner målet illustreres kommunikasjons algoritmen gjennom dronene som vil være et mobilt trådløst mesh nettverk gjennom en enkel nettapplikasjon.

Det vil også bli implementert en protokoll for å sende datapakkene pålitelig med UDP. Tanken her er å bruke Flooding algorithm. Det vil føre til at alle meldinger vil send til alle droner og dermed vil hele systemet være oppdatert og alle i søkemanskapet vil få informasjonene en trenger hvis de er koblet til nettverket.

## Implementert funksjonalitet

### Dronesimuleringssystem
Dronesimuleringssystemet er designet for å visualisere og styre droner som beveger seg mot et mål. Systemet er satt sammen av flere deler: en "simulator struct", individuelle droneprogrammer, og et frontend brukergrensesnitt.

### Simulator Struct
Simulator struct fungerer som et "falkeblikk" på dronene, og gir en oversikt over dronenes posisjoner og hvilke droner hver drone er koblet til. Denne informasjonen er representert som en graf, hvor hver drone og dens tilkoblinger er noder og kanter. For å unngå uønskede løkker er denne grafen forhåndsdefinert som et tre.

### Droneprogrammer
Hver drone er representert av et individuelt program. Dronene kommuniserer seg imellom ved hjelp av UDP-protokollen. En drone kan bare kommunisere med dronene den har direkte forbindelse med i grafen (nabo-dronene). Når simuleringen kjøres, åpnes en terminal for hver drone og det individuelle droneprogrammet kjøres.

### Frontend Brukergrensesnitt
Brukergrensesnittet viser dronene som blå prikker i et JavaScript-canvas, og målet som en rød prikk. Når simuleringen kjøres, beveger alle dronene seg mot målet. Et potensielt problem med dette er at droner nær målet kan ende opp med å overlappe posisjoner.

### Simuleringsprosessen
Simuleringen starter ved at frontend og simulator struct kjøres. Simulator struct kjører funksjonen run_drones, som igjen åpner terminaler og kjører droneprogrammene. Hver drone går inn i en loop der den venter på kommandoer.
Simulator struct fungerer som en server som mottar kommandoer fra frontend. Når frontend ber om et simuleringstrinn ved å kalle på do_step endepunktet, oppdaterer simulator struct posisjonene til dronene. Frontend henter deretter disse oppdaterte posisjonene og oppdaterer visningen i canvas.
Simuleringen sjekker kontinuerlig om noen av dronene har nådd målet. Hvis en drone når målet, stopper serveren og gir beskjed til frontend, som også stopper simuleringen.

## Nåværende løsning mot fremtidig
Applikasjonen er blitt planlagt og utviklet på et veldig kort tidsrom av to studenter. Det er lagt inn rundt 30+ timer fra hver student og det er dermed noen mangler og svakheter som vil bli drøftet under.

### Nåværende
Nåværende løsning har en del mangler opp mot den gitte problemløsningen. I problemløsningen blir det diskutert at det løsningen skal bruke UDP på en sikrere og mer påliteligmåte. Her er det ikke blitt gjort noen tiltak siden funksjonaliteten og en enkel simulering var hovedfokuset. 
Applikasjonen avsluttes i det en av dronene har nådd målet. Ideelt sett skulle dronene så returnert med informasjonene om hvor den bortkommende befant seg. 

Dronene flyr nå alle mot et punkt på canvaset i simuleringen. Ideelt sett skulle de flydd mot et område hvor den bortkommende kunne befinne seg. Deretter lete etter vedkommende, for så å returnere med informasjonen om vedkommende er funnet eller ikke osv.
I den nåværende løsningen har dronene sine naboer som de kan kommunisere med. Dronene har ingen mulighet til å oppdage nye kommunikasjons linjer på. Ideelt sett skulle dronene dynamisk kunne bryte og opprette kontakter med hverandre mens et mesh network alltid hadde blitt opprettholdt. Tanken var å implementere Ad hoc On-Demand Distance Vector (AODV) som rutingsprotokoll, men ble ikke gjennomført på grunn av tidsbegrensninger. 

### Fremtidig

For fremtidig utvikling av applikasjonen ville det blitt tatt tak i de største svakhetene nevnt over, nemlig mangel på AODV og en mer pålitelig bruk av UDP. UDP brukes jo nå for å ha minst mulig overhead mellom dronenes kommunikasjon. Men til en senere versjon av løsningen ville det vært bedre om protokollen hadde en mulighet for å kunne spørre på nytt om noe ikke er kommet frem eller at dronene kan svare med akklamasjon på at meldingen er mottatt. 

Den fremtidige løsningen ville også brukt AODV for å ha et dynamisk og flyttbart mesh network av droner. Droner ville da kunne oppdage nye kommunikasjonsruter og dermed hatt en mer dynamisk kommuniaksjonsflyt i stedet for den statiske kommunikasjonen mellom naboer ved bruk av flooding algorithm.

En ideell applikasjon er mer brukervennlig. Så i en senere versjon ville det vært ønskelig om applikasjonen ble mer tilpasset dette slik at det blir enklere for brukere av applikasjonen å starte den opp.
En fremtidig funksjonalitet som gjenre skulle blitt utviklet hadde vært å gjøre mesh-networket til et slags mobilt nettverk som evt. Brukere på bakken kan bruke til å kommunisere via dronene med hverandre. På den måten kan man opprettholde et dynamisk og mobilt nettverk for de som deltar under leteaksjonen.

## Eksterne avhengigheter

### Frontend
Frontend delen av applikasjonen er veldig enkel og har kun axios som avhengighet. Axios brukes til å sende http requester til backend for å kunne opdatere canvaset med lokasjonen til dronene i simuleringen.

### Backend

#### AODV – backend hovedapplikasjonen
-	Actix-cors: er en avhengighet som brukes i Actix-web-rammeverket for å legge til Cross-Origin Resource Sharing (CORS)-støtte. Actix-cors gir funksjonalitet for å konfigurere og håndtere CORS-policyer i Actix-web-applikasjoner.
-	Actix-web: er et asynkront web-rammeverk for Rust og brukes her hovedsakelig til håndtering av HTTP-forespørsler.
-	Serde: bruker til å serializere og derserializering av datastrukturer til JSON-format.
-	Serde-json: bruker til å serializere og derserializering av datastrukturer til JSON-format.
-	Regex: brukes til å finne mappe-lokasjonen til drone programmet. Ble hovedsakelig brukt i lag med dirs for å lese inn og starte opp dronene.
-	Dirs: er en avhengighet som gir en enkel måte å finne og håndtere kataloger og filbaner i Rust.

#### Drone – drone applikasjonen
-	Serde
-	Serde-json
Disse avhengighetene bruker til å serializere og derserializering av datastrukturer til JSON-format. Dette blir brukt til å sende og motta data i Json format, hovedsakelig koordinater.

## Installasjonsinstruksjoner

### Forutsetninger for å kjøre applikasjonen
-	Best om det brukes windows (er ikke nødvendig, men noe kode må kommenteres ut og gjøres manuelt for at det skal fungere)
-	Ha rust installert – last ned rust
-	Node versjon 18, som kan lastes ned fra: https://nodejs.org/en/download 
-	NPM (fås med nedlasting av Node)

### For å kjøre applikasjonens backend:
-	Åpne en tekst-terminal (CMD) og lokaliser prosjektets rot-mappen til prosjektet
-	Cd inn til backend sin rot mappe: cd aodv
-	Kjør kommandoen «cargo run» som kjører programmet. 
o	Programmet vil så åpne flere terminalvinduer.
o	Hvert vindu representerer en drone i simulasjonen.
o	PS: vil trolig kun fungere på windows. For å kjøre applikasjonen på noe annet se lengre ned …

### For å kjøre applikasjonens frontend:
-	Åpne en tekst-terminal (CMD) og lokaliser prosjektets rot-mappen til prosjektet
-	Cd inn til simulator mappen, cd simulator
-	Kjør kommandoen «npm install», for å installere avhengigheter
-	Deretter kjør kommandoen «npm run dev» som starter applikasjonen
-	Nå kan du finne applikasjonen på lenken: http://localhost:3000/ 
-	Da er det bare å ta i bruk applikasjonen

### Kjøring av backend på andre operativsystemer
-	Åpne en tekst-terminal (CMD) og lokaliser rot-mappen til prosjektet
-	Cd inn til simulator mappen, cd simulator
-	Åpne fem nye tekst-terminaler i drone-mappen lokalisert under rot-mappen, cd drone fra rot-mappen
-	Kjør kommandoen under i hver sin terminal
    o	Cargo run 0 240 325
    o	Cargo run 1 160 325
    o	Cargo run 2 309 396
    o	Cargo run 3 309 254
    o	Cargo run 4 100 369
-	Åpne opp enda en tekst-terminal i rot-mappen og cd inn til aodv, cd aodv
-	Kjør kommandoen «cargo run»
-	Etter en stund på ca. 10 sekunder er alt klart til å starte frontend. Se lenger opp under «For å kjøre applikasjonens frontend»

## Bruk av applikasjonen
-	Når backend suksessfullt er oppe og kjører og dronene er oppe og kjører kan man laste inn frontend på nytt.
-	Når du ser blå rundinger på frontend og «connected oppe i venstre hjørne er applikasjonen klar til å starte simuleringen
    o	Hvis du ikke ser det, kan du prøve å laste inn siden igjen.
-	Deretter er det bare å klikke på «Rescue simulation with drones» for å starte simuleringen.
-	Dronene vil da bevege seg mot den røde rundingen på andre siden av siden.
-	Den røde rundingen er da det området der den som er gått seg vill trolig vil befinne seg

