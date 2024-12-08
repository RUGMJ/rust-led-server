# Rust led server
> Led server written in rust

## Setup

### Pi zero w 1

1. Enable spi
2. Set spi buf siz to 8192
    `/etc/modprobe.d/spidev.conf`
    ```conf
    options spidev bufsiz=8192
    ```
3. Set core clock speed to 250
    `/boot/config.txt`
    ```conf
    # Snip
    core_freq=250
    # Snip
    ```

## Usage

`led-server {number of leds}`

### Example

`led-server 170`

