# WeatherCoin
WeatherCoin is a decentralized currency that relies on the randomness of the weather and cryptographic proofs as its foundation.
## How does it work?
Weather is a source of information where, regardless of who observes it, relatively consistent readings are obtained at the same time; however, it is impossible to predict its changes with absolute accuracy. By incorporating this characteristic of weather into the consensus mechanism of a decentralized system, we can create a currency that does not require proof-of-work.

WeatherCoin gets temperature data from multiple regions around the world. The regions are as follows:
```geojson
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          138.2529,
          36.2048,
          0
        ]
      },
      "properties": {
        "name": "36.2048, 138.2529",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          139.643815,
          35.452405,
          0
        ]
      },
      "properties": {
        "name": "35.452405, 139.643815",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          130.437317,
          33.586608,
          0
        ]
      },
      "properties": {
        "name": "33.586608, 130.437317",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          141.336388,
          43.079165,
          0
        ]
      },
      "properties": {
        "name": "43.079165, 141.336388",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.856765,
          38.253449,
          0
        ]
      },
      "properties": {
        "name": "38.253449, 140.856765",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          135.785899,
          35.060372,
          0
        ]
      },
      "properties": {
        "name": "35.060372, 135.785899",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          134.554346,
          34.075282,
          0
        ]
      },
      "properties": {
        "name": "34.075282, 134.554346",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.75725,
          41.797463,
          0
        ]
      },
      "properties": {
        "name": "41.797463, 140.757250",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "label-scale": 0,
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          139.643815,
          35.452405,
          0
        ]
      },
      "properties": {
        "name": "35.452405, 139.643815",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          130.437317,
          33.586608,
          0
        ]
      },
      "properties": {
        "name": "33.586608, 130.437317",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          141.336388,
          43.079165,
          0
        ]
      },
      "properties": {
        "name": "43.079165, 141.336388",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.856765,
          38.253449,
          0
        ]
      },
      "properties": {
        "name": "38.253449, 140.856765",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          135.785899,
          35.060372,
          0
        ]
      },
      "properties": {
        "name": "35.060372, 135.785899",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          134.554346,
          34.075282,
          0
        ]
      },
      "properties": {
        "name": "34.075282, 134.554346",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.75725,
          41.797463,
          0
        ]
      },
      "properties": {
        "name": "41.797463, 140.757250",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          140.446748,
          40.82189,
          0
        ]
      },
      "properties": {
        "name": "40.821890, 140.446748",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          133.196552,
          34.410487,
          0
        ]
      },
      "properties": {
        "name": "34.410487, 133.196552",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    },
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [
          137.954883,
          36.247005,
          0
        ]
      },
      "properties": {
        "name": "36.247005, 137.954883",
        "styleUrl": "#icon-1899-0288D1-nodesc",
        "icon-opacity": 1,
        "icon-color": "#0288d1",
        "icon-scale": 1,
        "icon-offset": [
          32,
          64
        ],
        "icon-offset-units": [
          "pixels",
          "insetPixels"
        ],
        "icon": "https://www.gstatic.com/mapspro/images/stock/503-wht-blank_maps.png"
      }
    }
  ]
}
```
