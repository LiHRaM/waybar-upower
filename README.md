# waybar-upower
DisplayDevice support on Waybar

Since Waybar does not support upower devices at the moment, I decided to implement it as an external module.

Use the following in `$HOME/.config/waybar/config` to get started:
```jsonc
  "custom/battery": {
    "format": "{}",
    "exec": "waybar-upower",
    "return-type": "json",
    "interval": 5
  }
```

I pair this with the following scss code:
```scss
#custom-battery {
  background: $base0B;
  &.charging {
    background: $base0A;
  }
  &.critical:not(.charging) {
    background: $base09;
    animation-name: blink;
    animation-duration: 0.5s;
    animation-timing-function: linear;
    animation-iteration-count: infinite;
    animation-direction: alternate;
  }
}

@keyframes blink {
  to {
    color: $base00;
    background: $base07;
  }
}
```
