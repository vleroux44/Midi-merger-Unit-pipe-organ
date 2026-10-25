# organologic.merge

`organologic.merge` is a native Max external placed between `notein` and `noteout`.
It forwards the first Note On for each `(channel, pitch)` pair, suppresses repeated
Note On messages, and forwards only the final matching Note Off.

## Max patching

Connect all three values in their natural order:

```text
[notein]
 pitch  velocity  channel
   |       |         |
[organologic.merge]
   |       |         |
 pitch  velocity  channel
   |       |         |
[noteout]
```

`notein` outputs channel and velocity before pitch. Pitch therefore triggers the
filter. The external likewise sends channel and velocity before sending pitch to
trigger `noteout`.

Messages accepted by the left inlet:

- `clear`: reset all counters without producing output.
- `panic`: output one Note Off for every active `(channel, pitch)` pair, then reset.

## Build on Windows

Requirements:

- Max 8.2 or newer;
- CMake 3.25 or newer;
- Visual Studio Build Tools with Desktop development with C++ and a Windows SDK;
- Git, used by CMake to fetch the pinned Cycling '74 Max SDK base.

From this directory:

```powershell
cmake -S . -B build -G "Visual Studio 17 2022" -A x64
cmake --build build --config Release
ctest --test-dir build -C Release --output-on-failure
```

With another Visual Studio release, replace the generator name with the one
listed by `cmake --help`. A single-configuration NMake build is also supported.

The external is generated in `externals/organologic.merge.mxe64`.

Copy the complete `max-external` directory into a Max Packages folder, for example:

```text
Documents/Max 9/Packages/organologic-merge
```

The same external is intended to run in both Max 8.6 and Max 9.x on Windows x64.
