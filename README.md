# Embassy Pico

Experiments with the [Embassy](https://embassy.dev/) framework on the Raspberry Pi Pico.

## Setup

### LED and Button

The board has a LED connected to GP22 and a button connected to GP0. When the button is
pressed, the LED will toggle. The implementation uses two Embassy tasks for this, one for
the toggling of the LED and one for the button handling. The communication between the
tasks is done using a channel. The button task also has a `count` variable. Each time the
button is pressed, the `count` variable is incremented. The value is sent to the LED task
and is printed when the LED is toggled.

### OLED SSD1306

The board has an OLED display with a SSD1306 controller that is connected to I2C. The
display is connected to GP17 (SCL) and GP16 (SDA). The handling of the display
is done using a third embassy task. When the button is pressed, the `count`
variable is sent also to the display task and the value is printed.

## Links

* [Embassy](https://embassy.dev/)
* [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp)

