API_RESP=$(curl -s "https://api.github.com/repos/filiptibell/lune/releases/latest")

for dir in ./*/
do
    dir=${dir%*/}
    dir=${dir##*/}

    arch=$(echo $dir | cut -d '_' -f2)

    case $arch in
      arm64)
        arch="aarch64"
        ;;
      x86-64|amd64)
        arch="x86_64"
        ;;
    esac

    dl_uri=$(grep -E "https.*lune-.*linux-$arch.zip" <<< $API_RESP | cut -d '"' -f4)

    wget $dl_uri -O $dir/usr/bin/lune
done

find . -maxdepth 1 -mindepth 1 -type d -exec dpkg --build {} \;
