[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

## Demo: Szukanie hasła do pliku zip

Na pewno każdemu czytelnikowi zdarzyło się otrzymać plik zaszyfrowany ich numerem PESEL. Być może komuś zależy na
uzyskaniu dostępu do takiego pliku, mimo, że nie jest jego adresatem. Moje narzędzie bardzo ułatwia to zadanie.

W tym przykładzie pokazuję, jak znaleźć hasło do pliku zip znając jedynie płeć i datę urodzenia adresata. Wystarczy
wykonać skrypt `zip_password.py` aby zobaczyć jak szybko można odnaleźć hasło do pliku.

#### Krok po kroku:

- Skrypt tworzy najpierw plik `encrypted.zip` zaszyfrowany hasłem `02070803628` (PESEL kobiety urodzonej w dniu
  08-07-1902). Zawartością archiwum jest `standard-document.pdf`.
    ```shell
    zip -P 02070803628 encrypted.zip standard-document.pdf
    ```

- Następnie tworzona jest wordlista na podstawie podanej płci i daty urodzenia.

    ```shell
    ../target/release/pesel-gen -g f -b 8-07-1902 > ./wordlist.txt
    ```

- Do znalezienia hasła na podstawie wordlisty wykorzystywany jest program `fcrackzip`.
    ```shell
    fcrackzip --use-unzip --dictionary --init-password ./wordlist.txt ./encrypted.zip
    ```
  Informacja o znalezieniu hasła powinna wyświetlić się w konsoli.


- W ostatnim kroku skrypt sprząta po sobie.
    ```shell
    rm ./wordlist.txt
    rm ./encrypted.zip
    ```