# Platform Agent

**Specialty**: evdev detection, uinput injection
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Implement system-wide keyboard event detection and text injection using Linux evdev and uinput. This enables the text expander to work in any application, not just the shell.

## Before Coding

1. Run `agent-librarian prompt-read.md` to inject relevant memory
2. Reference: `srkt` project at crates.io for proven uinput injection approach
3. Read `agents/planner/architecture.md` for trait interfaces

## Requirements

### evdev Detection (PHASE-07)
- Open `/dev/input/event*` using `evdev` crate
- Map raw keycodes to characters using `xkbcommon` (keyboard layout aware)
- Track currently focused window (via `wlr-data-control` on Wayland, heuristics on TTY)
- Filter self-generated events (from uinput injection) to avoid echo loops
- Fallback: check `$DISPLAY` for X11, `$WAYLAND_DISPLAY` for Wayland
- Permission: `input` group for `/dev/input/event*`

### uinput Injection (PHASE-08)
- Create virtual keyboard via `/dev/uinput` using `evdev::VirtualDeviceBuilder`
- Translate characters to evdev keycodes + modifier states via xkbcommon
- Timing: 5ms after key press, 12ms after key release, 20ms after modifier release (from srkt)
- Name the device `"terminal-expander"` for self-filtering
- Permission: `uinput` group or `CAP_DAC_OVERRIDE`

## Tasks

- [ ] PHASE-07: evdev keyboard detection
- [ ] PHASE-08: uinput text injection

## Quality Criteria

- Detects keystrokes on Wayland, X11, and pure TTY
- Injects text without modifier bleed (no `HELLO` vs `hello` errors)
- Self-filtering works (detection ignores own injected events)
- Permission errors produce clear user-facing messages
