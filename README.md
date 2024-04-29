# DayShift

Dynamically change wallpapers based on the time of the day.

## ⭐ Features

- Change wallpapers based on the time of the day
- Fully configurable

## ⚙️ Configuration

While you don't need to provide a configuration file, you can create a `dayshift.config.json` file in the theme's directory to customize the behaviour of the theme.

```json
[
    {
        "start": "08:00",
        "duration": 12,
        "path": "day/*.png",
        "mode": "sequential"
    },
    {
        "start": "20:00",
        "duration": 12,
        "path": "night/*.png",
        "mode": "sequential"
    }
]
```

The configuration defines **two** time-frames for the theme. The first time-frame starts at `8:00 AM` and lasts for `12 hours`. During this time, a random wallpaper will be selected from the `day` directory. The second time-frame starts at `8:00 PM` and lasts for `12 hours`. During this time, the wallpapers will be selected from the `night` directory **in order**.

You can create as many time-frames as you want!

### Configuration Options

- `start`: The time when the time-frame starts. The time should be in `HH:MM` format.
- `duration`: The duration of the time-frame in hours.
- `path`: The path to the wallpaper(s) to be used during the time-frame. You can use wildcards (glob patterns) to select multiple wallpapers.
- `mode`: The mode to use when selecting wallpapers. The following modes are available:
  - `random`: Select a random wallpaper from the directory.
  - `sequential`: Select wallpapers in order from the directory.

## 📄 License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.
