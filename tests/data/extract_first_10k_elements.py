import gzip
import fileinput


def main():
    counter = 0
    with gzip.open('uniparc_top10k.xml.gz', 'w') as fout:
        for line in fileinput.input(mode='rb'):
            fout.write(line)
            if line[:8] == b"</entry>":
                counter += 1
            if counter == 10000:
                fout.write(b"</uniparc>\n")
                break


if __name__ == '__main__':
    main()
