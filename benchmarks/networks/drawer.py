import operator
import re
from typing import List, Iterator, Dict, Tuple

from PIL import Image, ImageDraw


class GraphNode:
    def __init__(self, key_id: str, position: List[int]):
        self.key_id = key_id
        self.position = position


def draw_main_graph(
        network_logs: Dict[str, List[dict]], save_location: str) -> None:
    # This will make sure that `list-neighbours` is called at the end of
    # execution
    graph = list(__get_nodes(network_logs))
    neighbours = list(__get_neighbours(network_logs))
    __draw_graph(graph, neighbours, save_location)


def draw_query_graph(
        network_logs: Dict[str, List[dict]],
        key_id: str,
        message_id: str,
        save_location: str) -> None:
    graph = list(__get_nodes(network_logs))
    message_neighbours = list(__get_message_neighbours(
        network_logs[key_id], message_id))
    __draw_graph(graph, message_neighbours, save_location)


def __draw_graph(
        graph: List[GraphNode],
        neighbours: List[Tuple[str, str]],
        save_location: str):
    image_dims = [1920, 1080]
    location_dict = __get_location_dict(graph, image_dims)

    image = Image.new("RGBA", tuple(image_dims), color="white")
    draw = ImageDraw.Draw(image)

    __draw_neighbours(neighbours, location_dict, draw)
    __draw_nodes(graph, location_dict, draw)

    # Save the image
    image.save(save_location)


def __draw_nodes(
        graph: List[GraphNode],
        location_dict: Dict[str, Tuple[float, float]],
        draw: ImageDraw):
    """Draw all nodes as circles"""

    node_radius = 10
    for n in graph:
        (x, y) = location_dict[n.key_id]
        draw.ellipse(
            (
                x - node_radius,
                y - node_radius,
                x + node_radius,
                y + node_radius),
            fill="green")

    # Draw the key IDs next to the nodes
    # Done last to keep above node/neighbour drawings
    for n in graph:
        (x, y) = location_dict[n.key_id]
        draw.text((x, y), n.key_id, fill="black")


def __draw_neighbours(
        neighbours: List[Tuple[str, str]],
        location_dict: Dict[str, Tuple[float, float]],
        draw: ImageDraw):
    """Draw all neighbour connections"""

    for from_node, to_node in neighbours:
        (ax, ay) = location_dict[from_node]
        (bx, by) = location_dict[to_node]
        bidirectional_neighbour = (to_node, from_node) in neighbours

        if bidirectional_neighbour:
            # If both nodes of neighbours of each other, draw a green line
            draw.line((ax, ay, bx, by), fill="green", width=4)
        else:
            # If our neighbour does not have us as a neighbour, draw a half
            # green half red line, with the green half on this node's side
            (mx, my) = ((ax + bx) / 2, (ay + by) / 2)
            draw.line((ax, ay, mx, my), fill="green", width=4)
            draw.line((mx, my, bx, by), fill="red", width=4)


def __get_nodes(network_logs: Dict[str, List[dict]]) -> Iterator[GraphNode]:
    for key in network_logs:
        ns_logs = filter(
            lambda l: "neighbours_store" in l and l["neighbours_store"],
            network_logs[key])
        key_space_logs = map(operator.itemgetter("local_key_space"), ns_logs)
        groups = re.match(r"KeySpace\(([-0-9, ]+)\)", next(key_space_logs))
        key_space = list(map(int, groups.group(1).split(", ")))

        yield GraphNode(key, key_space)


def __get_neighbours(
        network_logs: Dict[str, List[dict]]) -> Iterator[Tuple[str, str]]:
    for key in network_logs:
        flags = ["list_neighbours", "reply"]
        neighbours_logs = list(filter(
            lambda l: all(map(lambda f: f in l and l[f], flags)),
            network_logs[key]))
        assert len(neighbours_logs) > 0, "Could not find logging of neighbours"
        neighbours = neighbours_logs[-1]["neighbours"]

        if neighbours == "":
            continue

        for n in neighbours.split(", "):
            yield (key, n)


def __get_message_neighbours(
        node_logs: List[dict],
        message_id: str) -> Iterator[Tuple[str, str]]:
    message_logs = [
        l for l in node_logs
        if "message_id" in l and l["message_id"] == message_id]

    for l in message_logs:
        if "found" not in l:
            continue
        key_id = __get_key_id_from_string(l["node"])
        neighbours = l["neighbours"].split(", ")
        yield from [(key_id, n) for n in neighbours]


def __get_location_dict(
        graph: List[GraphNode],
        image_dims: List[int]) -> Dict[str, Tuple[float, float]]:
    # Get the points and transpose them for bound calculation
    points = map(lambda n: n.position, graph)
    points_t = list(zip(*points))

    # Get the bounds of the dimensions
    unpadded_max_points = list(map(max, points_t))
    unpadded_min_points = list(map(min, points_t))

    # Add a padding of 10% around the bounds
    max_points = [
        _max + (_max - _min) * 0.1
        for _max, _min in zip(unpadded_max_points, unpadded_min_points)]
    min_points = [
        _min - (_max - _min) * 0.1
        for _max, _min in zip(unpadded_max_points, unpadded_min_points)]

    # Normalize the points within the bounds
    def normalize_point(ps: List[int]) -> Tuple[float, float]:
        normalized = [
            ((i - _min) / (_max - _min))
            for i, _max, _min in
            zip(ps, max_points, min_points)]
        assert len(normalized) == 2, \
            f"No support for drawing >2 dimensions, found {len(normalized)}"
        return tuple(
            float(i * dim) for i, dim in zip(normalized, image_dims))[:2]

    return dict((n.key_id, normalize_point(n.position)) for n in graph)


def __get_key_id_from_string(s: str) -> str:
    groups = list(re.finditer(r"Key\(([0-9A-F]+)\)", s))
    assert len(groups) == 1, f"Couldn't find exactly one key in {s}"
    key_id = groups[0].group(1)
    assert len(key_id) == 8, f"Found key that was not 8 long: {key_id}"
    return key_id