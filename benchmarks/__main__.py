"""Create, run, and manage collections of KIPA nodes in a KIPA network."""

import argparse
import logging
import os

from benchmarks import networks, utils


def main():
    parser = argparse.ArgumentParser("benchmarks")
    parser.add_argument(
        "-c",
        "--network_config",
        type=str,
        required=True,
        help="The file to read the benchmark configuration from")
    parser.add_argument(
        "-o",
        "--output_directory",
        type=str,
        default="benchmarks_output",
        help="Where to output benchmark results")

    args = parser.parse_args()
    configuration = networks.configuration.Configuration.from_yaml(
        args.network_config)
    configuration.run(
        os.path.join(
            args.output_directory,
            f"configuration_{utils.get_formatted_time()}"))


if __name__ == "__main__":
    logging.basicConfig()
    logging.getLogger().setLevel(logging.DEBUG)
    logging.getLogger("docker").setLevel(logging.WARNING)
    logging.getLogger("urllib3").setLevel(logging.WARNING)
    logging.getLogger("PIL").setLevel(logging.WARNING)
    main()
