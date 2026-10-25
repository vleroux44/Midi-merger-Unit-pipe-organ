# MIDI merger – Unit pipe organ

Utilitaires Windows pour consolider les messages MIDI Note On / Note Off
dupliqués, avec deux applications autonomes et un external pour Max.

## Contenu

Les versions prêtes à l’emploi sont dans [`dist/`](dist/).

- `dist/windows/MIDI merger - Unit pipe organ.exe` : application Rust avec
  interface graphique, permettant de choisir l’entrée et la sortie MIDI.
- `dist/windows/MIDI merger - Unit pipe organ (native).exe` : version native
  Windows, très compacte, pour le même usage.
- `dist/max/organologic.merge.mxe64` : external Windows x64 pour Max 8.2+
  et Max 9.

Le comportement est identique : pour une paire `(canal, note)`, seul le
premier Note On est transmis, puis seul le dernier Note Off correspondant.
Les autres messages MIDI sont relayés sans transformation.

## External Max

Copier le dossier `dist/max` dans un package Max, par exemple :

```text
Documents/Max 9/Packages/organologic-merge/externals/
```

Dans un patch, placer `organologic.merge` entre `notein` et `noteout` et
relier les trois flux dans l’ordre `pitch`, `velocity`, `channel`.
La documentation et le code de l’external sont dans
[`max-external/`](max-external/).

## Recompiler

### Applications Windows (Rust)

```powershell
cargo build --release --bin midi_merger
cargo build --release --bin midi_merger_native
```

Les exécutables générés se trouvent dans `target/release/`.

### External Max (C++)

Voir [`max-external/README.md`](max-external/README.md). Le SDK de base de
Cycling '74 est téléchargé automatiquement et verrouillé sur une révision
précise par CMake.

## Licence

Aucune licence de réutilisation n’est encore définie. Avant une publication
publique sur GitHub, ajouter une licence correspondant à tes choix (MIT,
GPL-3.0, propriétaire, etc.).
