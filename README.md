<h1 align="center">Fish Shoal Simulator</h1>

<p align="center">Interactive fish shoal simulator based on local rules to explore collective motion, phase transitions, and realistic
flock behaviors.</p>

## Downloads

|                                                           **Windows**                                                           |                                                         **Linux**                                                         |                                                         **MacOS**                                                         |
|:-------------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------------------------------:|
| [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-windows.exe) | [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-linux) | [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-macos) |

## Usage

### In-App Configuration

|    **Field** | **Value** |          **Range**          | **Description**                   |
|-------------:|:---------:|:---------------------------:|:----------------------------------|
| **Entities** | _Integer_ |       `0` → `10,000`        | The number of fish                |
|    **Width** | _Integer_ | `147` → `max screen width`  | The width of the simulation area  |
|   **Height** | _Integer_ | `200` → `max screen height` | The height of the simulation area |

### Additional Information

> The simulation world uses **toroidal wrapping**, meaning that entities exiting one edge of the screen reappear on the
> opposite edge, so the left and right borders are connected and the top and bottom borders are connected.

### App Demo

![fish-shoal-simulator.gif](screenshots/fish-shoal-simulator.gif)

## License

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
