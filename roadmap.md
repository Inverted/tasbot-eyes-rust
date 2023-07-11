# Roadmap

## Mandatory

- [x] Load GIFs into suitable data structure
- [x] Play GIF on LED matrix
- [x] 3 renderer types (with trait called Renderer)
  - [x] Debug console
  - [x] ~~WS2812 LED matrix~~ (no need, since I have the official LED board now)
  - [x] TASBot LED matrix
- [x] Color overwrite
- [x] Stack for animation queuing
- [x] Abstract program flow

## Higher priority

- [x] Blinking pattern
- [x] Configurable playback (speed, skip startup, repetitions, ...)
- [x] Configurable LED matrix (brightness, pin, RGB order, ...)
- [x] ~~UDP~~ TCP socket for injecting animations externally

## Low priority, some experimental

- [x] Color palette support
- [x] Playlist for animation
- [x] Gamma correction
- [ ] Renderer potentially running in independent thread
- [ ] Rainbow mode (dependent on render thread)
- [ ] WLED realtime control support
  - [ ] Partially (center LEDs)
  - [ ] Whole
