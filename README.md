# spear

spear is the unofficial Peacock project manager and "injector". We inject into the launcher window to display a new UI.
Settings as well as Peacock itself is managed inside spear.

# TODO

- fix settings so you can open and close it all the time instead of only once.
- add a slim, minimalism-like progress bar at the bottom right of the launcher window to indicate that we're downloading peacock (and till then of course disable the play with peacock button)
- make changing the peacock repo actually work

# Weird Finicky Things for Developers to Know

- The background resource needs to be a 4-channel (RGBA), non-interlaced image with 32 bits per pixel (8 bits per channel: Red, Green, Blue, Alpha) and of course its resolution: 608x344.