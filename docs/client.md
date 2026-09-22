# wallet-client

_all commands must be executed in the ./client folder_

## dependencies

### tauri dependencies

<details>
	<summary>Debian</summary>

```sh
sudo apt update
sudo apt install -y \
	libwebkit2gtk-4.1-dev \
	build-essential \
	curl \
	wget \
	file \
	libssl-dev \
	libgtk-3-dev \
	openjdk-21-jdk-headless \
	libayatana-appindicator3-dev \
	librsvg2-dev
```

</details>

<details>
	<summary>Arch</summary>

```sh
sudo pacman -Syu
sudo pacman -S --needed \
	webkit2gtk \
	base-devel \
	curl \
	wget \
	file \
	openssl \
	appmenu-gtk-module \
	gtk3 \
	libappindicator-gtk3 \
	librsvg \
	libvips
```

</details>

<details>
	<summary>Void</summary>

```sh
sudo xbps-install -Syu
sudo xbps-install -S \
	webkit2gtk-devel \
	curl \
	wget \
	file \
	openssl \
	gtk+3-devel \
	libappindicator \
	librsvg-devel \
	gcc \
	pkg-config
```

</details>

### node dependencies

```sh
npm install
```

## run

```sh
npm run dev
```

## run tauri version

```sh
npm run tauri dev
```
