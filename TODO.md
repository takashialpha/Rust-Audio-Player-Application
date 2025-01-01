# TODO - Audio Player Project

## Bug Fixes
- [x] Investigate and fix the issue where the first 3 seconds of audio are cut off.
- [ ] Remove warnings in the code.
- [ ] Remove unused and dead code.
- [ ] Fix the issue where the progress bar only update when play/pause/reboot, (not at real time).
- [ ] Fix the issue where panics when passes 100% of the song.

## Features
- [ ] Add support to automatically clean up temporary files at the end of execution.
- [x] Refactor the entire TUI module, which is currently not functional.
- [x] Improve TUI, adding progress bars, more features, and lock scrolling.
- [ ] Show file name, song duration in seconds (features in this scope).
- [ ] Integrate [Clap](https://docs.rs/clap/latest/clap/) for parsing external arguments.
- [x] Add support for playing MP3 files.
- [ ] Optimize the code in the `fstools` module.
- [ ] Improve the readability and maintainability of `fstools`.
- [ ] Add content to the `debug` module to support testing.
- [ ] Add support (keybind) for selecting other song while playing.
- [ ] Handle Interrupt (Ctrl+C).

