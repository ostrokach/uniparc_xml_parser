import fileinput
import gzip


def main(fin, fout, num_elements):
    counter = 0
    for line in fin:
        fout.write(line)
        if line[:8] == b"</entry>":
            counter += 1
        if counter == num_elements:
            fout.write(b"</uniparc>\n")
            break


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "input_files",
        default=None,
        nargs="?",
        help="input XML files (defaults to STDIN)",
    )
    parser.add_argument(
        "-k",
        "--num-elements",
        type=int,
        default=10_000,
        help="number of XML elements to extract.",
    )
    args = parser.parse_args()

    output_file = f"uniparc_top_{args.num_elements}.xml.gz"
    with fileinput.input(args.input_files or "-", mode="rb") as fin, gzip.open(
        output_file, mode="wb"
    ) as fout:
        main(fin, fout, args.num_elements)
