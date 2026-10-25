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
    "rect": [100.0, 100.0, 640.0, 420.0],
    "boxes": [
      {
        "box": {
          "id": "title",
          "maxclass": "comment",
          "text": "organologic.merge — first Note On, last matching Note Off",
          "fontsize": 18.0,
          "patching_rect": [30.0, 20.0, 560.0, 28.0]
        }
      },
      {
        "box": {
          "id": "description",
          "maxclass": "comment",
          "text": "Connect pitch, velocity and channel between notein and noteout.",
          "patching_rect": [30.0, 55.0, 500.0, 22.0]
        }
      },
      {
        "box": {
          "id": "notein",
          "maxclass": "newobj",
          "text": "notein",
          "numinlets": 1,
          "numoutlets": 3,
          "outlettype": ["int", "int", "int"],
          "patching_rect": [220.0, 105.0, 100.0, 22.0]
        }
      },
      {
        "box": {
          "id": "merge",
          "maxclass": "newobj",
          "text": "organologic.merge",
          "numinlets": 3,
          "numoutlets": 3,
          "outlettype": ["int", "int", "int"],
          "patching_rect": [190.0, 180.0, 160.0, 22.0]
        }
      },
      {
        "box": {
          "id": "noteout",
          "maxclass": "newobj",
          "text": "noteout",
          "numinlets": 3,
          "numoutlets": 0,
          "patching_rect": [220.0, 255.0, 100.0, 22.0]
        }
      },
      {
        "box": {
          "id": "clear",
          "maxclass": "message",
          "text": "clear",
          "numinlets": 2,
          "numoutlets": 1,
          "patching_rect": [40.0, 145.0, 45.0, 22.0]
        }
      },
      {
        "box": {
          "id": "panic",
          "maxclass": "message",
          "text": "panic",
          "numinlets": 2,
          "numoutlets": 1,
          "patching_rect": [95.0, 145.0, 45.0, 22.0]
        }
      },
      {
        "box": {
          "id": "footer",
          "maxclass": "comment",
          "text": "© 2026 Valentin Leroux — www.organologic.fr",
          "patching_rect": [30.0, 365.0, 400.0, 22.0]
        }
      }
    ],
    "lines": [
      { "patchline": { "source": ["notein", 0], "destination": ["merge", 0] } },
      { "patchline": { "source": ["notein", 1], "destination": ["merge", 1] } },
      { "patchline": { "source": ["notein", 2], "destination": ["merge", 2] } },
      { "patchline": { "source": ["merge", 0], "destination": ["noteout", 0] } },
      { "patchline": { "source": ["merge", 1], "destination": ["noteout", 1] } },
      { "patchline": { "source": ["merge", 2], "destination": ["noteout", 2] } },
      { "patchline": { "source": ["clear", 0], "destination": ["merge", 0] } },
      { "patchline": { "source": ["panic", 0], "destination": ["merge", 0] } }
    ]
  }
}
