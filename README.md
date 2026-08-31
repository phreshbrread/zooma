# zooma
Basic screen zoomer heavily inspired by [boomer](https://github.com/tsoding/boomer), intended for all X11 and Wayland environments.

### Controls
| Action | Control |
| ---    | ---     | 
| Pan around image | ``Left click`` + drag |
| Spotlight effect | ``Ctrl`` |
| Adjust spotlight size | ``Ctrl`` + ``shift`` + ``scroll`` up / down |
| Zoom in / out | ``Scroll`` up / down |
| Reset image position & zoom and spotlight size | ``R`` |


### Runtime Dependencies
- ``scrot`` (for X11)
- ``grim`` (for Wayland)
- ``spectacle`` (optional, for KDE Wayland)
- ``flameshot`` (optional, for GNOME Wayland)
- ``glfw``
- ``libgl``
- ``libx11``
- ``libxcursor``
- ``libxi``
- ``libxinerama``
- ``libxkbcommon``

### Build Dependencies
- ``glfw``
- ``cmake``
- ``clang``
