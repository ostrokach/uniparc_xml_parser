from pathlib import Path
from typing import List

import pyarrow as pa
import pyarrow.parquet as pq
from pyarrow import csv
from tqdm import tqdm


def csv_to_parquet(
    csv_file: Path, parquet_file: Path, *, delimiter: str, column_names: List[str]
) -> None:
    block_size = 1 << 24  # 16 MB
    read_options = csv.ReadOptions(column_names=column_names, block_size=block_size)
    parse_options = csv.ParseOptions(delimiter=delimiter)
    writer = None
    with csv.open_csv(
        csv_file, read_options=read_options, parse_options=parse_options
    ) as csv_reader:
        for batch in tqdm(csv_reader):
            if writer is None:
                writer = pq.ParquetWriter(parquet_file, csv_reader.schema, compression="zstd")
            table = pa.Table.from_batches([batch])
            writer.write_table(table)
    if writer is not None:
        writer.close()


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-f", "--input-file", help="input CSV file")
    parser.add_argument("-c", "--column-names", help="names of columns")
    parser.add_argument("-d", "--delimiter", default="\t", help="delimiter used by the CSV file")
    parser.add_argument("-o", "--output-file", default=None, help="output Parquer file")

    args = parser.parse_args()

    csv_file = Path(args.input_file).resolve(strict=True)
    parquet_file = (
        Path(args.output_file).resolve(strict=True)
        if args.output_file is not None
        else csv_file.with_suffix(".parquet")
    )
    delimiter = args.delimiter
    column_names = args.column_names.split(",")

    csv_to_parquet(csv_file, parquet_file, delimiter=delimiter, column_names=column_names)
