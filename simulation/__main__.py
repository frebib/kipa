"""Create, run, and manage collections of KIPA nodes in a KIPA network."""

import argparse
import logging
from pathlib import Path

import yaml

from simulation import utils, benchmarks
from simulation.key_creator import KeyCreator
from simulation.networks import Network
from simulation.operations.simulator import simulate

log = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser("simulation")
    parser.add_argument(
        "-c",
        "--network_config",
        type=str,
        required=True,
        help="The file to read the simulation configuration from",
    )
    parser.add_argument(
        "-o",
        "--output_directory",
        type=str,
        default="simulation_output",
        help="Where to output simulation results",
    )
    parser.add_argument(
        "--comparison",
        type=str,
        choices=["angle"],
        default=None,
        help="Run a comparison of the performance on a variable",
    )
    parser.add_argument(
        "--benchmark",
        type=str,
        choices=["reliability", "resilience", "speed", "scalability"],
        default=None,
        help="Run a benchmark to see how well a configuration performs under "
        "varying conditions",
    )

    args = parser.parse_args()
    network_config = Path(args.network_config)
    output_directory = Path(args.output_directory)

    key_creator = KeyCreator()
    with open(str(network_config), "r") as file:
        network = Network.from_config(yaml.load(file), key_creator)

    if args.benchmark is not None:
        if args.benchmark == "reliability":
            benchmark = benchmarks.ReliabilityBenchmark(output_directory)
        elif args.benchmark == "resilience":
            benchmark = benchmarks.ResilienceBenchmark(output_directory)
        elif args.benchmark == "speed":
            benchmark = benchmarks.SpeedBenchmark(output_directory)
        elif args.benchmark == "scalability":
            benchmark = benchmarks.ScalabilityBenchmark(output_directory)
        else:
            raise ValueError(f"Unrecognized benchmark type: {args.benchmark}")
        log.info(f"Running {args.benchmark} benchmark")
        benchmark.create(network)
        return

    log.info("Running configuration")
    simulate(network, output_directory / "cli" / utils.get_formatted_time())


if __name__ == "__main__":
    logging.basicConfig()
    logging.getLogger().setLevel(logging.DEBUG)
    logging.getLogger("docker").setLevel(logging.WARNING)
    logging.getLogger("urllib3").setLevel(logging.WARNING)
    logging.getLogger("PIL").setLevel(logging.WARNING)
    main()
