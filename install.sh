#!/bin/sh

if ! command -v curl > /dev/null 2>&1; then
  echo "Execution interruption: curl is not installed"
  exit 1
fi

URL="https://github.com/smokingplaya/bashhistory/releases/latest/download/bashhistory-linux_x86"
OUTPUT="bashhistory"

curl -o $OUTPUT $URL
sudo chmod +x $OUTPUT
sudo mv $OUTPUT /usr/local/bin
echo "The file was successfully moved to /usr/local/bin"
