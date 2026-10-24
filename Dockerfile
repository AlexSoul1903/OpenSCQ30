# syntax=docker/dockerfile:1
FROM rust:1.99.1-trixie AS development
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libdbus-1-dev libxkbcommon-dev libwayland-dev libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
RUN rustup component add rustfmt
ENV CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0

FROM development AS build
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo test --locked -p openscq30-gui --features bundled-dependencies \
    && cargo build --locked -p openscq30-gui --features bundled-dependencies \
    && cp target/debug/openscq30-gui /usr/local/bin/openscq30-gui \
    && cosmic_revision="$(awk '/^name = "libcosmic"/{found=1} found && /^source =/{sub(/.*#/, ""); print substr($0,1,7); exit}' Cargo.lock)" \
    && test -n "$cosmic_revision" \
    && set -- /usr/local/cargo/git/checkouts/libcosmic-*/"$cosmic_revision"/cosmic-icons \
    && mkdir -p /opt/cosmic-icons \
    && cp "$1/index.theme" "$1/LICENSE" "$1/COPYING" /opt/cosmic-icons/ \
    && cp -a "$1/freedesktop/scalable" /opt/cosmic-icons/ \
    && cp -a "$1/extra/scalable/." /opt/cosmic-icons/scalable/

FROM debian:trixie-slim AS preview
RUN apt-get update && apt-get install -y --no-install-recommends \
    xvfb x11vnc novnc websockify openbox dbus-x11 x11-utils \
    libdbus-1-3 libxkbcommon0 libwayland-client0 libwayland-cursor0 libwayland-egl1 \
    libvulkan1 mesa-vulkan-drivers libegl1 libgl1 fonts-noto-core fonts-noto-color-emoji \
    adwaita-icon-theme curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 1000 studio
RUN install -d -o studio -g studio /home/studio/.config \
    && install -d -m 1777 /tmp/.X11-unix \
    && apt-get update && apt-get install -y --no-install-recommends libxkbcommon-x11-0 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/bin/openscq30-gui /usr/local/bin/openscq30-gui
COPY --from=build /opt/cosmic-icons /usr/share/icons/Cosmic
COPY docker/entrypoint.sh /usr/local/bin/openscq30-preview
RUN chmod +x /usr/local/bin/openscq30-preview
USER studio
ENV DISPLAY=:99 XDG_RUNTIME_DIR=/tmp/runtime-studio LANG=en_US.UTF-8 \
    WINIT_UNIX_BACKEND=x11 WGPU_BACKEND=vulkan LIBGL_ALWAYS_SOFTWARE=1 \
    OPENSCQ30_DEMO=1 \
    RUST_LOG=warn,cosmic_config::dbus=off
EXPOSE 6080
HEALTHCHECK --interval=15s --timeout=3s --start-period=30s \
    CMD curl --fail --silent http://127.0.0.1:6080/vnc.html >/dev/null || exit 1
ENTRYPOINT ["/usr/local/bin/openscq30-preview"]
