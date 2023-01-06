Oxide Emulator Project
----------------------

Oxidemu is aimed to emulate retro consoles

## Status
Work in progress. At the moment Chip8 emulation is under development

## Dependencies
SDL2.dll should be available in system path or be copied to project dir. [VCPKG](https://github.com/microsoft/vcpkg) can be used to install lib files (see installation manual). Set env variables for lib files
```
RUSTFLAGS=-L path-to-vcpkg/installed/x64-windows/lib
```