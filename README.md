# Embassy Pico

Experiments with the [Embassy](https://embassy.dev/) framework on the Raspberry Pi Pico.

## Setup

The board has a LED connected to GP22 and a button connected to GP0. When the button is
pressed, the LED will toggle. The implementation uses two Embassy tasks, one for the
toggling of the LED and one for the button handling. The communication between the tasks
is done using a channel. The button task also has a `count` variable. Each time the button
is pressed, the `count` variable is incremented. The value is sent to the LED task and is
printed when the LED is toggled.

## Links

* [Embassy](https://embassy.dev/)
* [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp)

