#!/usr/bin/bash


path="$HOME/.pdfviewer"

uninstall() {
    sudo rm /usr/local/bin/pdfviewer
    rm -fr $path
}

install() {
    echo "Installing binary"
    if [ -d ./target ]; then rm -fr ./target; fi
    cargo build --release

    echo "Linking bin to /usr/local/bin"
    sudo ln -P ./target/release/pdfviewer /usr/local/bin/

    echo "Creating path: ${path}"
    mkdir -p $path

    echo "Coping static assets"
    cp -r ./ui $path

    echo "Done"
}

case "$1" in
    i)
        install
        ;;
    x)
        uninstall
        ;;
    *)
        echo "usage: sh ./installation.sh [-i|-x] for installation/uninstallation"
        ;;
esac
