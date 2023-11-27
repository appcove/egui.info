
# Examples

Each example, which is part of the [git repository](https://github.com/appcove/egui.info.git) which runs this site, is *fully runnable rust code*. 

Once you have rust installed (see the [setup page](/setup)), here is how to run any example:

```bash
# Do this once to get the code
git clone https://github.com/appcove/egui.info.git

# Enter the egui.info directory
cd egui.info

# Examples can be built with
cargo build -p egui-101-basic
# And run with
cargo run -p egui-101-basic
```


## 101: Basic Examples

1. [egui-101-basic](./egui-101-basic): A heading, label, and button.
1. [egui-101-menu](./egui-101-menu): A menu bar with several menus
1. [egui-101-moving-circle](./egui-101-moving-circle): A circle which moves across the screen

## 112: Basic Interactions

1. [egui-112-circle-follow-mouse](./egui-112-circle-follow-mouse): Following the mouse pointer with a circle
1. [egui-112-button-move-circle](./egui-112-button-move-circle): Buttons that effect a circle on the screen
1. [egui-112-keypress-move-circle](./egui-112-keypress-move-circle): Moving a circle around the screen using keypresses

## 122: Basic Widgets

1. [egui-122-checkbox-functionality](./egui-122-checkbox-functionality) Using Checkboxes to change the circle color
1. [egui-122-slider](./egui-122-slider) Using Sliders to change the circle color
1. [egui-122-combo-box](./egui-122-combo-box) A simple example of an egui combo box
1. [egui-122-button-grid](./egui-122-button-grid) A example of buttons, sliders, and grids in egui
1. [egui-122-multiple-choice](./egui-122-multiple-choice) A multiple choice/radio button example
1. [egui-122-text-size](./egui-122-text-size) Text size example in egui
1. [egui-122-check-box](./egui-122-check-box) A widget for check boxes in egui

## 144: Basic Interactive Games

1. [egui-144-clicker-game](./egui-144-clicker-game): Gets points for clicks
1. [egui-144-color-clicker](./egui-144-color-clicker): Four circles that you have to turn off
1. [egui-144-circle-chaser](./egui-144-circle-chaser/): A circle and a target, Touch the target
