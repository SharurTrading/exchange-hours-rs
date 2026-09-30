#!/bin/zsh
url="$1"; shift
for i in 1 2 3 4 5 6; do
  out=$(curl -s --max-time 60 "http://web.archive.org/cdx/search/cdx?url=${url}&output=text&$*")
  if ! printf '%s' "$out" | grep -q "Temporarily Offline"; then
    printf '%s' "$out"
    exit 0
  fi
  sleep $((i*5))
done
printf 'CDX-FAILED\n'
