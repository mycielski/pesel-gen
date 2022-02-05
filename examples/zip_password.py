"""Demo: finding PESEL-encrypted zip archive's password,
knowing the intended recipient's date of birth and gender."""
import os


def main():
    """main()"""
    os.system("zip -P 02070803628 encrypted.zip standard-document.pdf")
    os.system("../target/release/pesel-gen -g f -b 8-07-1902 > ./wordlist.txt")
    os.system(
        "fcrackzip --use-unzip --dictionary --init-password ./wordlist.txt ./encrypted.zip"
    )
    os.system("rm ./wordlist.txt")
    os.system("rm ./encrypted.zip")


if __name__ == "__main__":
    main()
