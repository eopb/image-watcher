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

| Name            | global | local | type |
| :-------------- | :----: | :---: | ---: |
| width           |  [ ]   |  [x]  |      |
| height          |  [ ]   |  [x]  |      |
| blur            |  [ ]   |  [x]  |      |
| sharpen         |  [ ]   |  [x]  |      |
| adjust_contrast |  [ ]   |  [x]  |      |
| brighten        |  [ ]   |  [x]  |      |
| huerotate       |  [ ]   |  [x]  |      |
| flipv           |  [ ]   |  [x]  |      |
| fliph           |  [ ]   |  [x]  |      |
| rotate90        |  [ ]   |  [x]  |      |
| rotate180       |  [ ]   |  [x]  |      |
| rotate270       |  [ ]   |  [x]  |      |
| grayscale       |  [ ]   |  [x]  |      |
| invert          |  [ ]   |  [x]  |      |




resize
blur
sharpen
adjust_contrast
brighten
huerotate
flipv
fliph
rotate90
rotate180
rotate270
grayscale
invert

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

## TODO