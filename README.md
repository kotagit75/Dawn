<div align="center">
    <h1>Dawn</h1>
</div>

Dawn is a decentralized currency that relies on the randomness of the weather and cryptographic proofs as its foundation.

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

![BG](./assets/bg.jpg)

> [!NOTE]
> Dawn is currently in active development. The API and features may change without notice.

## :sparkles: Features
- ⛅ Consensus by Weather - Weather data enables rapid consensus building
- ⚡ Highly energy-efficient - Because VDF is used instead of Proof of Work, it is more energy-efficient

## :dart: How does it work?
Weather is a source of information where, regardless of who observes it, relatively consistent readings are obtained at the same time; however, it is impossible to predict its changes with absolute accuracy. By incorporating this characteristic of weather into the consensus mechanism of a decentralized system, we can create a currency that does not require proof-of-work.

Here, data that is difficult to predict is referred to as a "beacon." Dawn uses a hash chain composed of linked blocks. In addition to transactions, each block contains the beacon and the results of VDF calculations. Because the value of the beacon is difficult to predict, it is challenging to generate future blocks. Furthermore, the hash chain is employed to prevent the generation of blocks using past beacon values.
## :rocket: Get started
### Installation
```bash
# Clone the repository (or Download ZIP)
git clone git@github.com:kotagit75/Dawn.git

# Navigate to the project directory
cd Dawn

# build
cargo build --release
```
### Create a script to retrieve the temperature
```bash
mkdir beacon
```
Create a shell script named `beacon/temperature.sh`.This script retrieves the latitude and longitude and returns the temperature at that location as an exit code.It doesn't matter how you implement it. Here is an example. Note that this API does not actually exist.
```bash
#!/bin/bash
temperature=$(curl "https://example.com/api?latitude=$1&longitude=$2")
exit $(echo "scale=1; $temperature * 10" | bc | sed s/\.[0-9,]*$//g)
```
Even without using an API, it is possible to conduct observations by placing sensors on-site, for example.
### Usage
```bash
# run
./target/release/dawn

# get state
curl localhost:8080/state

# get balance
curl localhost:8080/balance

# send transaction
curl -X POST -H "Content-Type: application/json" -d "{'recipient':'{address}', 'amount': {amount}}" localhost:8080/tx

# mine a block
curl -X POST localhost:8080/mine

# add peer
curl -X POST -H "Content-Type: application/json" -d '{"ip":"{IP Addr}"}' localhost:8080/peer
```

## 📍 Locations which is collected temperature data
Dawn gets temperature data from multiple regions. The regions are as follows:
- Hakodate `140.7290611111111, 41.76869722222222`
- Hirosaki `140.4845583333333, 40.599217`
- Sendai `140.8694166666667, 38.26819444444445`
- Yokohama `139.6379638888889, 35.443675`
- Nagoya `136.9065583333334, 35.18145`
- Kyoto `135.76815, 35.01156388888889`
- Kobe `135.1956305555556, 34.69008055555555`
- Hiroshima `132.4553055555556, 34.38528888888889`
- Fukuoka `130.4016888888889, 33.59018333333334`
- Kagoshima `130.55, 31.5667`

[View geojson](src/beacon/target.geojson)

```geojson
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          130.4016888888889,
          33.59018333333334,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=4",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "08A4CA3ABB3E9772BB90"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.7290611111111,
          41.76869722222222,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "0530A314AA3E9773BBCD"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          139.6379638888889,
          35.443675,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "071C1FDBF13E9774F7C7"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          135.76815,
          35.01156388888889,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "0BC66EC3F63E9775D992"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.8694166666667,
          38.26819444444445,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "0A0F43D3643E97839B19"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          135.1956305555556,
          34.69008055555555,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "0F489967AA3E97841D2B"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          136.9065583333334,
          35.18145,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "0EDF57EBE63E978A0E80"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          132.4553055555556,
          34.38528888888889,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=40",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      },
      "id": "092C7EF2BA3E978AA3E6"
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.4845583333333,
          40.599217,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=400",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          130.55,
          31.5667,
          0
        ]
      },
      "properties": {
        "styleUrl": "#msn_icon?color=fbc02d&id=2000&scale=400",
        "fill-opacity": 0.25098039215686274,
        "fill": "#ffffff",
        "stroke-opacity": 1,
        "stroke": "#fbc02d",
        "stroke-width": 4,
        "label-scale": 0.8,
        "icon-scale": 1.5,
        "icon-offset": [
          64,
          128
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://earth.google.com/earth/document/icon?color=fbc02d&id=2000&scale=4"
      }
    }
  ]
}
```
