# win-syscolor

This crate provides a safe wrapper around the `GetSysColor` function. To get a color, call `SysColor::get`. The available colors are listed in the `SysColorIndex` enum.

## Examples

```
use win_syscolor::{SysColor, SysColorIndex};

let color = SysColor::get(SysColorIndex::ActiveCaption).expect("Color not available");
println!("The active caption color is {}", color);
```

## Dependency Justification

This crate only depends on `windows-sys`, which it uses to interface with the Windows API.

## License

This project is dual-licensed under the Boost Software License version 1.0 and the Apache 2.0 License. See the `LICENSE-APACHE` and `LICENSE-BOOST` files for details.
