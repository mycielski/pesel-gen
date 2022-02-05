#!/bin/bash

zip -P 02070803628 encrypted.zip standard-document.pdf

../target/release/pesel-gen -g f -b 8-07-1902 > ./wordlist.txt

fcrackzip --use-unzip --dictionary --init-password ./wordlist.txt ./encrypted.zip

rm ./wordlist.txt
rm ./encrypted.zip
