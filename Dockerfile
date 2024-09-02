ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

RUN apt update && \
    apt install -y curl && \
    curl -L https://github.com/mozilla/sccache/releases/download/v0.7.7/sccache-v0.7.7-x86_64-unknown-linux-musl.tar.gz | tar xzf -

ENV TZ=Etc/UTC
COPY ./build/*.sh /chromium_build/
COPY ./build/install-build-deps.py /chromium_build/
RUN \
	DEBIAN_FRONTEND=noninteractive \
	echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections \
	&& ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone \
	&& apt-get update && apt-get install -y lsb-release sudo \
	&& sed -i 's/snapcraft/snapcraftnoinstall/g' /chromium_build/install-build-deps.py \
	&& /chromium_build/install-build-deps.sh --no-prompt --no-chromeos-fonts \
	&& apt-get --just-print install binutils binutils-aarch64-linux-gnu binutils-arm-linux-gnueabihf binutils-mips64el-linux-gnuabi64 binutils-mipsel-linux-gnu bison bzip2 cdbs curl dbus-x11 devscripts dpkg-dev elfutils fakeroot flex git-core gperf lib32z1 libasound2 libasound2-dev libatk1.0-0 libatspi2.0-0 libatspi2.0-dev libbluetooth-dev libbrlapi-dev libbrlapi0.7 libbz2-1.0 libbz2-dev libc6 libc6-dev libcairo2 libcairo2-dev libcap-dev libcap2 libcgi-session-perl libcups2 libcups2-dev libcurl4-gnutls-dev libdrm-dev libdrm2 libegl1 libelf-dev libevdev-dev libevdev2 libexpat1 libffi-dev libffi7 libfontconfig1 libfreetype6 libfuse2 libgbm-dev libgbm1 libgl1 libglib2.0-0 libglib2.0-dev libglu1-mesa-dev libgtk-3-0 libgtk-3-dev libinput-dev libinput10 libjpeg-dev libkrb5-dev libncurses6 libnspr4 libnspr4-dev libnss3 libnss3-dev libpam0g libpam0g-dev libpango-1.0-0 libpangocairo-1.0-0 libpci-dev libpci3 libpcre3 libpixman-1-0 libpng16-16 libpulse-dev libpulse0 libsctp-dev libspeechd-dev libspeechd2 libsqlite3-0 libsqlite3-dev libssl-dev libstdc++6 libsystemd-dev libudev-dev libudev1 libuuid1 libva-dev libvulkan-dev libvulkan1 libwayland-egl1 libwayland-egl1-mesa libwww-perl libx11-6 libx11-xcb1 libxau6 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxdmcp6 libxext6 libxfixes3 libxi6 libxinerama1 libxkbcommon-dev libxrandr2 libxrender1 libxshmfence-dev libxslt1-dev libxss-dev libxt-dev libxtst-dev libxtst6 lighttpd locales mesa-common-dev openbox p7zip patch perl pkgconf rpm ruby subversion uuid-dev wdiff x11-utils x11-xserver-utils xcompmgr xserver-xorg-core xserver-xorg-video-dummy xvfb xz-utils zip zlib1g zstd \
	&& rm -rf /chromium_build \
	&& rm -rf /var/lib/apt/lists/*

RUN chmod +x /sccache-v0.7.7-x86_64-unknown-linux-musl/sccache

ENV SCCACHE=/sccache-v0.7.7-x86_64-unknown-linux-musl/sccache
ENV SCCACHE_DIR=./target/sccache
