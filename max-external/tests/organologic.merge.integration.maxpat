{
  "patcher": {
    "fileversion": 1,
    "appversion": {
      "major": 8,
      "minor": 6,
      "revision": 5,
      "architecture": "x64",
      "modernui": 1
    },
    "classnamespace": "box",
    "rect": [100.0, 100.0, 700.0, 500.0],
    "boxes": [
      { "box": { "id": "lb", "maxclass": "newobj", "text": "loadbang", "patching_rect": [40.0, 30.0, 65.0, 22.0] } },
      { "box": { "id": "ton", "maxclass": "newobj", "text": "t b b b", "patching_rect": [40.0, 75.0, 65.0, 22.0] } },
      { "box": { "id": "channel", "maxclass": "message", "text": "1", "patching_rect": [190.0, 115.0, 30.0, 22.0] } },
      { "box": { "id": "velon", "maxclass": "message", "text": "100", "patching_rect": [115.0, 115.0, 40.0, 22.0] } },
      { "box": { "id": "noteson", "maxclass": "message", "text": "60 60 60 60 60", "patching_rect": [20.0, 115.0, 90.0, 22.0] } },
      { "box": { "id": "iteron", "maxclass": "newobj", "text": "iter", "patching_rect": [45.0, 150.0, 35.0, 22.0] } },
      { "box": { "id": "doff", "maxclass": "newobj", "text": "delay 50", "patching_rect": [285.0, 75.0, 60.0, 22.0] } },
      { "box": { "id": "toff", "maxclass": "newobj", "text": "t b b", "patching_rect": [285.0, 115.0, 50.0, 22.0] } },
      { "box": { "id": "veloff", "maxclass": "message", "text": "0", "patching_rect": [350.0, 150.0, 30.0, 22.0] } },
      { "box": { "id": "notesoff", "maxclass": "message", "text": "60 60 60 60 60", "patching_rect": [250.0, 150.0, 90.0, 22.0] } },
      { "box": { "id": "iteroff", "maxclass": "newobj", "text": "iter", "patching_rect": [275.0, 185.0, 35.0, 22.0] } },
      { "box": { "id": "merge", "maxclass": "newobj", "text": "organologic.merge", "numinlets": 3, "numoutlets": 3, "outlettype": ["int", "int", "int"], "patching_rect": [120.0, 240.0, 160.0, 22.0] } },
      { "box": { "id": "pack", "maxclass": "newobj", "text": "pack 0 0 0", "patching_rect": [120.0, 295.0, 90.0, 22.0] } },
      { "box": { "id": "print", "maxclass": "newobj", "text": "print ORGAN_TEST", "patching_rect": [120.0, 340.0, 125.0, 22.0] } }
    ],
    "lines": [
      { "patchline": { "source": ["lb", 0], "destination": ["ton", 0] } },
      { "patchline": { "source": ["lb", 0], "destination": ["doff", 0] } },
      { "patchline": { "source": ["ton", 2], "destination": ["channel", 0] } },
      { "patchline": { "source": ["channel", 0], "destination": ["merge", 2] } },
      { "patchline": { "source": ["ton", 1], "destination": ["velon", 0] } },
      { "patchline": { "source": ["velon", 0], "destination": ["merge", 1] } },
      { "patchline": { "source": ["ton", 0], "destination": ["noteson", 0] } },
      { "patchline": { "source": ["noteson", 0], "destination": ["iteron", 0] } },
      { "patchline": { "source": ["iteron", 0], "destination": ["merge", 0] } },
      { "patchline": { "source": ["doff", 0], "destination": ["toff", 0] } },
      { "patchline": { "source": ["toff", 1], "destination": ["veloff", 0] } },
      { "patchline": { "source": ["veloff", 0], "destination": ["merge", 1] } },
      { "patchline": { "source": ["toff", 0], "destination": ["notesoff", 0] } },
      { "patchline": { "source": ["notesoff", 0], "destination": ["iteroff", 0] } },
      { "patchline": { "source": ["iteroff", 0], "destination": ["merge", 0] } },
      { "patchline": { "source": ["merge", 2], "destination": ["pack", 2] } },
      { "patchline": { "source": ["merge", 1], "destination": ["pack", 1] } },
      { "patchline": { "source": ["merge", 0], "destination": ["pack", 0] } },
      { "patchline": { "source": ["pack", 0], "destination": ["print", 0] } }
    ]
  }
}
