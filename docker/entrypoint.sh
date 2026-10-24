#!/bin/sh
set -eu
umask 077
mkdir -p "$XDG_RUNTIME_DIR" "$HOME/.config/openscq30"
chmod 700 "$XDG_RUNTIME_DIR"
# Persist a random viewer password without placing it in logs or Compose's environment.
password_file="$HOME/.config/openscq30/preview-password"
if [ ! -s "$password_file" ]; then
    python3 -c 'import secrets; print(secrets.token_urlsafe(6))' > "$password_file"
fi
chmod 600 "$password_file"
x11vnc -storepasswd "$(cat "$password_file")" "$XDG_RUNTIME_DIR/vnc-password" >/dev/null 2>&1
if ! xdpyinfo -display "$DISPLAY" >/dev/null 2>&1; then
    rm -f /tmp/.X99-lock /tmp/.X11-unix/X99
fi
Xvfb "$DISPLAY" -screen 0 1440x900x24 -nolisten tcp &
xvfb_pid=$!
cleanup() {
    kill "$xvfb_pid" 2>/dev/null || true
    wait "$xvfb_pid" 2>/dev/null || true
}
trap cleanup EXIT INT TERM
for attempt in $(seq 1 50); do
    if xdpyinfo -display "$DISPLAY" >/dev/null 2>&1; then break; fi
    sleep 0.1
done
xdpyinfo -display "$DISPLAY" >/dev/null
openbox >/tmp/openbox.log 2>&1 &
# VNC stays internal; Compose exposes only the browser viewer on localhost.
x11vnc -display "$DISPLAY" -forever -shared -rfbauth "$XDG_RUNTIME_DIR/vnc-password" -localhost -rfbport 5900 >/tmp/vnc.log 2>&1 &
# Reject cross-site WebSocket requests, including browser requests from other local ports.
websockify --web=/usr/share/novnc --file-only --auth-plugin=ExpectOrigin \
    --auth-source='http://localhost:6080 http://127.0.0.1:6080' \
    6080 localhost:5900 >/tmp/websockify.log 2>&1 &
dbus-run-session -- openscq30-gui
