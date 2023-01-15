FROM gitpod/workspace-full-vnc
USER root
RUN apt update && \
    apt install -yq libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev && \
    npm i -g {{pkg_manager}}
USER gitpod
