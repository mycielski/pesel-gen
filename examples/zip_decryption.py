import os


def main():
    os.system("zip -P 02070803628 encrypted.zip standard-document.pdf")
    print("Created encrypted zip file from standard-document.pdf")
    os.system(f"../target/release/pesel-gen -g f -b 8-07-1902 > ./wordlist.txt")
    print("Generated wordlist")
    os.system(f"fcrackzip --use-unzip --dictionary --init-password ./wordlist.txt ./encrypted.zip")
    os.system("rm ./wordlist.txt")
    os.system("rm ./encrypted.zip")
    print("Cleaned files.")


if __name__ == '__main__':
    main()
