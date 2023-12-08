// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::Icon;

pub fn tray_icon() -> Icon {
    Icon::Raw(include_bytes!("../../icons/tray-icon-256.png").to_vec())
}

#[cfg(target_os = "macos")]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_0.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_1.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_2.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_3.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_4.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_5.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_6.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_7.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_8.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_9.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_10.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_11.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_12.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_13.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_14.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_15.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_16.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_17.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_18.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_19.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_20.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_21.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_22.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_23.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_24.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_25.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_26.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_27.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_28.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/macos/loading_29.png").to_vec(),
    ));
    icon_vec
}

#[cfg(not(target_os = "macos"))]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_0.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_1.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_2.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_3.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_4.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_5.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_6.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_7.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_8.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_9.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_10.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_11.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_12.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_13.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_14.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_15.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_16.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_17.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_18.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_19.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_20.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_21.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_22.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_23.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_24.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_25.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_26.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_27.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_28.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../assets/windows/loading_29.ico").to_vec(),
    ));
    icon_vec
}
