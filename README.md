# LV2 XDO

This plugin receives LV2 control messages (which you can MIDI-learn in Carla,
for example) and redirects them as keyboard shortcuts with libxdo (like xdotool)
to whatever application receives them (e.g. your media player).

## Dependencies

```bash
# Required libraries.
sudo apt-get install lv2-dev libxdo-dev

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build

To produce the deployable `lv2xdo.lv2` output directory.

```bash
./build.sh
```

## Development

To rebuild and try the plugin in Carla:

```bash
./develop.sh
```
