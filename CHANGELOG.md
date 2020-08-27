# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## Unreleased

* Rename `TextureHolder` to `TextureRef`.
* Modify `ElementBuffer` type to `u16` to improve performance.

## 0.0.3 (2020-08-21)

* Add graphics transform.
* Rewrite transform.
* Rewrite draw mesh.
* Add prelude.
* Fix text layout wrap calculation.

## 0.0.2 (2020-08-07)

* Add font and text layout support.
* Adjust draw params.
* Rewrite window dpi logic.
* Add modifiers change support to keyboard module.
* Use 1*1 white texture as default when there is no texture.

## 0.0.1 (2020-03-06)

Initial release!

The basic framework include:
* Window.
* Event loop and timer.
* Vertex and sprite rendering.
* Keyboard, mouse, touch, touchpad and gamepad input handling.
