# `image-watcher`

## What is it

`image-watcher` is a CLI tool that "compiles" images using various transforms set in a configuration file to produce new transformed images.
`image-watcher` also has the option to "watch" source images for changes and update the transformed images with every new change.

## Why did I make it

I was developing a static website that used Pug and SASS to generate HTML and CSS.
I needed to find a way to, in a somewhat automated way, form low-resolution images from the high-resolution images in the repository.
I could not find a program that offered a simple solution to my problem so I made my own program.

---

## How to install

Run

```
cargo install image-watcher
```

with cargo installed

## Set up

Make a file called `image_watcher.yaml` in your repository.

Here is an example of what it can look like.

```yaml
# Global settings are set here and apply to all images
grayscale: true

# Each file is set in this list. Each file can have their own transforms.
files:
  -
    path: 'private\images\backgrounds\hand_and_book.JPG'
    height: 350
  -
    path: 'private\images\backgrounds\goldcrest.jpg'
    height: 400
  -
    path: 'private\images\backgrounds\downs.jpg'
    width: 1000
  -
    path: 'private\images\backgrounds\light.jpg'
    width: 1000
```

The transforms available include.

| Name            | global | local |                Description                 |  type   |                                   unit                                    |
| :-------------- | :----: | :---: | :----------------------------------------: | :-----: | :-----------------------------------------------------------------------: |
| path            |        |   ✓   |            Sets path of image.             | String  |                               Relative path                               |
| output          |        |   ✓   |     Sets path to save output image to.     | String  |                               Relative path                               |
| width           |   ✓    |   ✓   | Sets width while preserving aspect ratio.  | Integer |                                  Pixels                                   |
| height          |   ✓    |   ✓   | Sets height while preserving aspect ratio. | Integer |                                  Pixels                                   |
| resize_filter   |   ✓    |   ✓   |      Sets filter used when resizing.       | String  | `"Nearest"` / `"Triangle"` / `"CatmullRom"` / `"Gaussian"` / `"Lanczos3"` |
| blur            |   ✓    |   ✓   |           Gaussian blurs image.            |  Float  |                                   Sigma                                   |
| sharpen         |   ✓    |   ✓   |              Sharpens image.               | Integer |                                                                           |
| adjust_contrast |   ✓    |   ✓   |          Changes image contrast.           |  Float  |          Negative values decrease and  positive values increase           |
| brighten        |   ✓    |   ✓   |              Brightens image.              | Integer |                            Amount to brighten                             |
| huerotate       |   ✓    |   ✓   |             Rotates image hue.             | Integer |                                  Degrees                                  |
| flipv           |   ✓    |   ✓   |          Flips image vertically.           | Boolean |                              `True`/`False`                               |
| fliph           |   ✓    |   ✓   |         Flips image horizontally.          | Boolean |                              `True`/`False`                               |
| rotate90        |   ✓    |   ✓   |         Rotates image 90 degrees .         | Boolean |                              `True`/`False`                               |
| rotate180       |   ✓    |   ✓   |         Rotates image 180 degrees.         | Boolean |                              `True`/`False`                               |
| rotate270       |   ✓    |   ✓   |         Rotates image 270 degrees.         | Boolean |                              `True`/`False`                               |
| grayscale       |   ✓    |   ✓   |           Makes image grayscale.           | Boolean |                              `True`/`False`                               |
| invert          |   ✓    |   ✓   |               Inverts image.               | Boolean |                              `True`/`False`                               |



## CLI options

There are two CLI options `--compile` and `--watch`.

Navigate to your repository in your command line

Run
```
image-watcher --compile
```
to compile the images one time.

or

Run
```
image-watcher --watch
```
to compile the images and watch for changes to them.

---

## Downloads

### Stable

[Windows](https://gitlab.com/efunb/image-watcher/-/jobs/artifacts/stable/raw/files/image-watcher.exe?job=windows-optimized) 

[Linux](https://gitlab.com/efunb/image-watcher/-/jobs/artifacts/stable/raw/files/image-watcher?job=linux-optimized) 

### Nightly

[Windows](https://gitlab.com/efunb/image-watcher/-/jobs/artifacts/master/raw/files/image-watcher.exe?job=windows-optimized) 

[Linux](https://gitlab.com/efunb/image-watcher/-/jobs/artifacts/master/raw/files/image-watcher?job=linux-optimized) 

## TODO