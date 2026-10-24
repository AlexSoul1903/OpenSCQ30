# Audio Studio UI

The desktop app is a native Rust/COSMIC (iced) application. There is no CSS or
web frontend. `gui/src/theme.rs` owns the design tokens and native widget styles.

## Visual rules

- Shell and window canvas: Prussian Blue `#011936`.
- Cards and panels: Charcoal Blue `#465362`.
- Main text, readouts and slider handles: Mint Cream `#F4FFFD`.
- Alerts, destructive actions and battery levels at or below 20%: Watermelon Red `#ED254E`.
- Selected modes, active tracks and EQ response/nodes: Royal Gold `#F9DC5C`.
- Use gold with blue text and red with mint text. Pair alerts with a written label.
- Use a 16px spacing rhythm, 24px page padding and rounded 18px cards.
- Sora is bundled under the SIL Open Font License in `gui/resources/fonts/OFL.txt`.
  The OpenSCQ30 Metrics face is a semibold derivative with the `tnum` glyph
  substitutions baked into its cmap, giving numeric readouts equal-width digits.
  Regenerate it with `python gui/resources/fonts/build_metrics.py` (requires fonttools).

Audio Studio is the default for new configurations. Existing system/light/dark
preferences remain available in Settings > Theme. Select Audio Studio there if
an existing configuration chooses a different theme.

The device list and short mode choices stack on smaller windows. EQ band controls
use two columns when at least 640px are available and one column otherwise. Long
settings pages scroll. The native navigation drawer handles narrow windows.

## EQ interaction

The gold curve is the device's EQ response, not a live spectrum analyzer.
Custom profiles support node dragging and named native sliders/spin buttons for
keyboard and assistive technology access. Preset profiles expose the response and
numeric values without editable handles. Gain is clamped to the device's bounds.
The curve uses bounded Bezier segments; preset updates and node hover rings use
a 180ms cubic-bezier(0.22, 1, 0.36, 1) transition. Direct dragging follows the cursor.
Animation redraws stop when the transition finishes.
Settings > Reduce animations disables these transitions and persists the preference.

## Run in Docker

From the repository root:

```sh
docker compose up --build -d
```

Open <http://localhost:6080/vnc.html?autoconnect=1&resize=scale> and connect to the
demo Soundcore Life Q30. This runs the actual Linux desktop binary through Xvfb
and noVNC. A simulated device is registered through the existing demo backend when
`OPENSCQ30_DEMO=1`; simulated hardware state stays within that marked demo device.

The preview binds only to localhost and persists settings in a Docker volume.
The viewer requires a generated password. Retrieve it locally with:

```sh
docker compose exec studio cat /home/studio/.config/openscq30/preview-password
```

Enter that password in noVNC when prompted. It is stored with owner-only file
permissions and is not printed in service logs. The WebSocket accepts only the
localhost/127.0.0.1 viewer origins on port 6080. Keep the loopback port binding;
this development preview is not configured for remote hosting or TLS.
The container drops Linux capabilities and prevents privilege escalation.
The build runs GUI unit tests before producing the binary. BuildKit caches Rust
dependencies and build artifacts for subsequent builds.

```sh
docker compose logs -f studio
docker compose stop
```

Docker Desktop's Linux container does not automatically share Windows Bluetooth
or Windows audio. This setup is for UI development and demo interactions. Run the
native Windows binary for physical headphones (see `docs/build-windows.md`).
Linux hardware operation requires a separately configured BlueZ/system D-Bus
connection; the preview Compose file does not expose one. Docker's
[USB/IP documentation](https://docs.docker.com/desktop/features/usbip/) describes
advanced device forwarding and its platform constraints.

For a Rust development shell without installing Rust on Windows:

```sh
docker build --target development -t openscq30-development .
```

Mount this repository at `/app` when running that image. `cargo fmt --all`,
`cargo check --locked -p openscq30-gui --features bundled-dependencies` and
`cargo test --locked -p openscq30-gui --features bundled-dependencies` are the
desktop validation commands.
