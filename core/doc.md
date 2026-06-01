## `core/` (`image_core`) — Documentation

**Cargo.toml:** Package `image_core` v0.1.0, depends on `image = "0.24"`.

### `lib.rs` — Module root
Re-exports 5 public modules: `getinfo`, `includes`, `kernel`, `operation`, `transformcolor`.

---

### `includes.rs` — Core types & arithmetic
| Item | Description |
|---|---|
| `RawImage` struct | Flat `Vec<f32>` pixel data + `x_size`, `y_size`, `channels`. Implements `Display`, `Clone`, `Add`, `Sub`, `Mul<RawImage>` (component-wise), `Mul<f32>` (scaling). |
| `Pixel` struct | `r, g, b, a: f32` — 4-channel representation. |
| `img_power(img)` | Element-wise square of pixel data. |
| `img_sqrt(img)` | Element-wise square root of pixel data. |
| `is_valid_coord(image, x, y)` | Bounds check. |
| `access_pixel_at_coord(image, x, y)` | Reads pixel at `(x, y)`. Handles 1-channel (all RGB set to same value) and 3-channel layouts; alpha hardcoded to `1.0`. Unsupported channels return black. |

---

### `kernel.rs` — Convolution kernels
| Item | Description |
|---|---|
| `normalize_kernel(kernel)` | Normalizes a 2D kernel so the sum of all coefficients is 1.0. |
| `apply_kernel(image, kernel)` | Convolves a **single-channel** `RawImage` with a 2D kernel. Clamps output to `[0.0, 1.0]`. Returns a 1-channel image. |
| `make_gaussian_kernel(size, sigma)` | Generates a Gaussian blur kernel. |
| `apply_gaussian_blur(image, sigma, size)` | Convenience: makes the kernel and applies it. |
| `apply_sobel_x(image)` | Sobel X edge detection (3×3 kernel `[-1,0,1; -2,0,2; -1,0,1]`). |
| `apply_sobel_y(image)` | Sobel Y edge detection (3×3 kernel `[-1,-2,-1; 0,0,0; 1,2,1]`). |
| `apply_sobel(image)` | Combined Sobel: magnitude = `sqrt(Gx² + Gy²)`. |

---

### `getinfo.rs` — Image statistics & histograms
All functions assume **single-channel** images (assert `channels == 1`).

| Item | Description |
|---|---|
| `get_image_info(image)` | Returns tuple `(mean, median, max, min, stddev, width, height)`. |
| `get_image_info_string(image)` | Human-readable summary string of the above. |
| `get_image_mean(image)` | Arithmetic mean of pixel values. |
| `get_image_stddev(image, mean)` | Population standard deviation (mean passed in). |
| `get_image_max(image)` / `get_image_min(image)` | Max/min pixel value. |
| `get_image_median(image)` | Median pixel value (clones & sorts data). |
| `get_image_histogram(image)` | Maps luminance bucket `u8` (value × 255) to pixel count `u32`. |
| `get_image_histogram_normalized(histogram)` | Converts counts to frequencies (`f32`). |
| `print_histogram(...)` / `print_histogram_normalized(...)` | Prints sorted (by count) histogram to stdout. |

---

### `transformcolor.rs` — Color & geometric transforms
| Item | Description |
|---|---|
| `make_grasyscale(image)` | Averages RGB channels → single-channel output. Note: function name is a misspelling of "grayscale". |
| `rotate_image(image, angle)` | Rotates by 0°, 90°, 180°, or 270°. Returns `Result<RawImage, String>`. |
| `rotate90(image)` | Clockwise 90° rotation. Transposes dimensions. |
| `rotate180(image)` | 180° rotation (upside-down). Same dimensions. |
| `rotate270(image)` | Clockwise 270° rotation (equivalent to 90° counter-clockwise). |
| `change_pixel_at_coord(image, x, y, pixel)` | Writes a `Pixel` into the buffer at `(x, y)`. Supports 1- and 3-channel layouts. Panics on invalid coords. |

---

### `operation.rs` — Empty
The file `core/src/operation.rs` exists but is empty.
