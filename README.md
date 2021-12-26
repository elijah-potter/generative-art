# Generative Art

This is my personal project for making Generative Art.

## Structure

There are two crates: 

* generative-art: algorithms for generation.
* ga-web: a web interface for the project. You can try it out on [my website](https://elijahpotter.dev/art).

### Generative art

Right now, there are just three generators:

* Preslav: the Rust implementation of Preslav Rachev's book *Generative Art in Go*.
* Celestial: simulates and renders the motion of celestial objects.
* Wave: runs across an image, drawing sine waves at the frequency of a specific part of the image.

![Example of Preslav generation](./example_images/preslav.svg)
![Example of celestial generation](./example_images/celestial.svg)
![Example of waves generation](./example_images/output3.png)
