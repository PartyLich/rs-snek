The classic game of Snake, with a few added features. OOP-y sort of structure on
master, ECS-like structure on refactor/simplest-ecs branch.

## Building

```
cargo build --release
```

## Building for windows from a linux box

```
# mingw SDL2
tar xf ~/Downloads/SDL2-devel-2.0.12-mingw.tar.gz -C ~/Downloads/SDL2mingw
mkdir $_
tar xf ~/Downloads/SDL2-devel-2.0.12-mingw.tar.gz -C ~/Downloads/SDL2mingw
cp -r  ~/Downloads/SDL2mingw/SDL2-2.0.12/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/

# mingw SDL2_TTF
tar xf ~/Downloads/SDL2_ttf-devel-2.0.15-mingw.tar.gz -C ~/Downloads/SDL2_TTFmingw
mkdir $_
tar xf ~/Downloads/SDL2_ttf-devel-2.0.15-mingw.tar.gz -C ~/Downloads/SDL2_TTFmingw
cp -r  ~/Downloads/SDL2_TTFmingw/SDL2_ttf-2.0.15/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/

# with cargo
cargo build --release --target x86_64-pc-windows-gnu
# or with cross
cross build --release --target x86_64-pc-windows-gnu
```

## Running

Executable directory should have the binary, SDL2 dlls (the build instruction above is not
statically linked), and the resource folder (maps, fonts). At some point maybe I'll get around to
either setting up a CI build or at least uploading a zip to the releases. Maybe.

Maps are a simple csv format. A 36x36 grid with '1's for walls/obstacles. A random map is selected
each time Labyrinth mode is initiated. Naming scheme is important, eg map_00.csv.
