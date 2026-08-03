### Installation
```bash
# Clone the repository (or Download ZIP)
git clone https://github.com/kotagit75/btfy.git

# Navigate to the project directory
cd btfy
```

### Create a script to retrieve the temperature
Create a script. This script reads latitude, longitude, and timestamp from stdin and writes the temperature to stdout as JSON. It doesn't matter how you implement it.
Even without using an API, it is possible to conduct observations by placing sensors on-site, for example.
#### Examples
- [example/dummy.sh](../example/dummy.sh)
- [example/open-meteo.sh](../example/open-meteo.sh)
- [example/aviation-weather.sh](../example/aviation-weather.sh)

### Run
Run btfy.
```bash
cargo run -- --mining --beacon-cmd example/open-meteo.sh
```
