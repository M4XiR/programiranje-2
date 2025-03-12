# Generiki in značilnosti

Za naloge, ki sledijo, boste potrebovali strukturo aritmetičnega zaporedja,
ki ste ga definirali zadnjič in strukturo izrazov in binarnih operacij.

- Popravite definicijo aritmetičnega zaporedja od zadnjič, da bo delovala za poljubne tipe `T`. Ali imajo vse metode od zadnjič smisel?
- Ugotovi, katerim značilnostim mora zadoščati tip `T`, da bodo imele metode od zadnjič smisel. Za tak tip `T`, popravi implementacijo metod od zadnjič. Zmnožek dveh aritmetičnih zaporedij definiramo tako, da zmnožimo začetna člena in diferenci (zato da bo produkt dveh aritmetičnih zaporedij tudi aritmetično zaporedje).
- Implementirajte značilnost `PartialEq` za aritmetična zaporedja.

- Popravite `Izraz` tako, da bo konstanta v izrazu poljubnega tipa `T`.
- Katerim značilnostim mora zadoščati tip `T`, da bo imela metoda `eval` smisel? Kaj pa `collect ` in `izpis`?
- Za `Izraz` implementirajte značilnost `ToString`.

- Definicijo aritmetičnega zaporedja ponovno spremenite tako, da bodo metode uporabljale samo `Izraz`-e in `BinOperacija`. Če je treba za zaporedja definirati dodatno značilnost, jo definirajte. Se da to narediti na lep način? Ali lahko za `Izraz` implementiramo značilnost `Copy`?

- Ustvarite nekaj aritmetičnih zaporedij, in testirajte operacije na njih.
