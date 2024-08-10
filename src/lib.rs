use battery::{self};
use jay_config::{
    input::{acceleration::ACCEL_PROFILE_FLAT, capability::CAP_TOUCH},
    keyboard::{
        parse_keymap,
        syms::{
            SYM_XF86AudioMute, SYM_backslash, SYM_c, SYM_p, SYM_s, SYM_space, SYM_7, SYM_8, SYM_9,
        },
        Keymap,
    },
    on_idle,
};
use uom::{fmt::DisplayStyle::*, si::f32::*, si::time::minute};

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use {
    chrono::{format::StrftimeItems, Local},
    jay_config::{
        config,
        exec::set_env,
        exec::Command,
        get_workspace,
        input::{get_seat, input_devices, on_new_input_device, InputDevice, Seat},
        keyboard::{
            mods::{Modifiers, ALT, CTRL, MOD4, SHIFT},
            syms::{
                SYM_Return, SYM_XF86AudioLowerVolume, SYM_XF86AudioRaiseVolume,
                SYM_XF86MonBrightnessDown, SYM_XF86MonBrightnessUp, SYM_b, SYM_d, SYM_e, SYM_f,
                SYM_h, SYM_i, SYM_j, SYM_k, SYM_l, SYM_m, SYM_q, SYM_r, SYM_slash, SYM_t, SYM_u,
                SYM_v, SYM_x, SYM_1, SYM_2, SYM_3, SYM_4, SYM_5, SYM_6, SYM_F1, SYM_F10, SYM_F11,
                SYM_F12, SYM_F2, SYM_F3, SYM_F4, SYM_F5, SYM_F6, SYM_F7, SYM_F8, SYM_F9, SYM_y
            },
        },
        quit, reload,
        status::set_status,
        switch_to_vt,
        timer::{duration_until_wall_clock_is_multiple_of, get_timer},
        video::on_graphics_initialized,
        Axis::{Horizontal, Vertical},
        Direction::{Down, Left, Right, Up},
    },
    std::{cell::RefCell, time::Duration},
};

const MOD: Modifiers = MOD4;
fn configure_seat(s: Seat) {
    let keymap: Keymap = parse_keymap(include_str!("keymap.xkb"));
    s.set_keymap(keymap);
    s.bind(MOD | SYM_h, move || s.focus(Left));
    s.bind(MOD | SYM_j, move || s.focus(Down));
    s.bind(MOD | SYM_k, move || s.focus(Up));
    s.bind(MOD | SYM_l, move || s.focus(Right));

    s.bind(MOD | SHIFT | SYM_h, move || s.move_(Left));
    s.bind(MOD | SHIFT | SYM_j, move || s.move_(Down));
    s.bind(MOD | SHIFT | SYM_k, move || s.move_(Up));
    s.bind(MOD | SHIFT | SYM_l, move || s.move_(Right));

    s.bind(MOD | SYM_d, move || s.create_split(Horizontal));
    s.bind(MOD | SHIFT | SYM_s, move || s.create_split(Vertical));

    s.bind(MOD | SYM_s, move || s.toggle_split());
    s.bind(MOD | SYM_m, move || s.toggle_mono());
    s.bind(MOD | SYM_f, move || s.toggle_fullscreen());

    s.bind(MOD | SYM_t, move || s.focus_parent());

    s.bind(MOD | SHIFT | SYM_q, move || s.close());

    s.bind(MOD | SYM_space, move || s.toggle_floating());

    s.bind(MOD | SHIFT | SYM_Return, || {
        Command::new("alacritty").spawn()
    });

    s.bind(MOD | SYM_e, || {
        Command::new("/home/uncomfy/.config/river/emoji.sh").spawn()
    });

    s.bind(MOD | SYM_y, || {
        Command::new("/home/uncomfy/.local/bin/yt-cli.nu").spawn()
    });

    s.bind(MOD | SYM_c, || {
        Command::new("/home/uncomfy/.config/river/clipboardmanager.sh").spawn()
    });

    s.bind(MOD | SYM_u, || Command::new("fuzzel").spawn());

    s.bind(MOD | SYM_v, || {
        Command::new("jay")
            .arg("run-privileged")
            .arg("--")
            .arg("tessen")
            .arg("-d")
            .arg("fuzzel")
            .arg("-a")
            .arg("copy")
            .spawn()
    });

    s.bind(MOD | SYM_slash, || {
        Command::new("/home/uncomfy/.config/river/book.nu").spawn()
    });

    s.bind(MOD | SYM_backslash, || {
        Command::new("/home/uncomfy/.config/river/fnottctl_list.sh").spawn()
    });

    s.bind(MOD | SYM_b, || {
        Command::new("/home/uncomfy/.config/river/nubrowser.nu").spawn()
    });

    s.bind(MOD | SYM_i, || {
        Command::new("flatpak")
            .arg("--user")
            .arg("run")
            .arg("org.mozilla.firefox")
            .spawn()
    });

    s.bind(MOD | SYM_p, || {
        Command::new("/home/uncomfy/.local/bin/mygrimshot.sh").spawn()
    });

    s.bind(MOD | SHIFT | SYM_p, || {
        Command::new("/home/uncomfy/.local/bin/mygrimshot.sh")
            .arg("area")
            .spawn()
    });

    s.bind_masked(Modifiers::NONE, SYM_XF86MonBrightnessUp, move || {
        Command::new("/usr/bin/brightnessctl")
            .arg("s")
            .arg("+4%")
            .spawn()
    });

    s.bind_masked(Modifiers::NONE, SYM_XF86MonBrightnessDown, move || {
        Command::new("/usr/bin/brightnessctl")
            .arg("s")
            .arg("4%-")
            .spawn()
    });

    s.bind_masked(Modifiers::NONE, SYM_XF86AudioRaiseVolume, move || {
        Command::new("wpctl")
            .arg("set-volume")
            .arg("@DEFAULT_SINK@")
            .arg("1%+")
            .spawn()
    });

    s.bind_masked(Modifiers::NONE, SYM_XF86AudioLowerVolume, move || {
        Command::new("wpctl")
            .arg("set-volume")
            .arg("@DEFAULT_SINK@")
            .arg("1%-")
            .spawn()
    });

    s.bind_masked(Modifiers::NONE, SYM_XF86AudioMute, move || {
        Command::new("wpctl")
            .arg("set-mute")
            .arg("@DEFAULT_SINK@")
            .arg("toggle")
            .spawn()
    });

    s.bind(MOD | SHIFT | SYM_x, quit);

    s.bind(MOD | SHIFT | SYM_r, reload);

    // let use_hc = Cell::new(true);
    // s.bind(MOD | SHIFT | SYM_m, move || {
    //     let hc = !use_hc.get();
    //     use_hc.set(hc);
    //     log::info!("use hc = {}", hc);
    //     s.use_hardware_cursor(hc);
    // });

    let fnkeys = [
        SYM_F1, SYM_F2, SYM_F3, SYM_F4, SYM_F5, SYM_F6, SYM_F7, SYM_F8, SYM_F9, SYM_F10, SYM_F11,
        SYM_F12,
    ];

    for (i, sym) in fnkeys.into_iter().enumerate() {
        s.bind(CTRL | ALT | sym, move || switch_to_vt(i as u32 + 1));
    }

    let numkeys = [
        SYM_1, SYM_2, SYM_3, SYM_4, SYM_5, SYM_6, SYM_7, SYM_8, SYM_9,
    ];

    for (i, sym) in numkeys.into_iter().enumerate() {
        let ws = get_workspace(&format!("{}", i + 1));
        s.bind(MOD | sym, move || s.show_workspace(ws));
        s.bind(MOD | SHIFT | sym, move || {
            s.set_workspace(ws);
            s.show_workspace(ws);
        });
    }

    s.use_hardware_cursor(false);

    // fn do_grab(s: Seat, grab: bool) {
    //     for device in s.input_devices() {
    //         if device.has_capability(CAP_KEYBOARD) {
    //             log::info!(
    //                 "{}rabbing keyboard {:?}",
    //                 if grab { "G" } else { "Ung" },
    //                 device.0
    //             );
    //             grab_input_device(device, grab);
    //         }
    //     }
    //     if grab {
    //         s.unbind(SYM_y);
    //         s.bind(MOD | SYM_c, move || do_grab(s, false));
    //     } else
    //         s.unbind(MOD | SYM_c);
    //         s.bind(SYM_y, move || do_grab(s, true));
    //     }
    // }
    // do_grab(s, false);
}

fn check_battery() -> battery::Result<battery::Battery> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            // Thin wrapper from battery crate
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to find any batteries",
            )
            .into());
        }
    };
    Ok(battery)
}

// TODO wireplumber.rs
fn get_wireplumber() -> String {
    let vol = std::process::Command::new("wpctl")
        .arg("get-volume")
        .arg("@DEFAULT_SINK@")
        .output()
        .expect("Command failed");

    unsafe { String::from_utf8_unchecked(vol.stdout) }
}

fn get_brightness() -> f32 {
    let backlight: String = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("brightnessctl")
                .arg("get")
                .output()
                .expect("Command failed")
                .stdout,
        )
    }
    .trim()
    .into();
    let backlight = backlight.parse::<f32>().expect("Cannot parse value");
    backlight / 240_f32
}

fn setup_status() -> Result<(), battery::Error> {
    let time_format: Vec<_> = StrftimeItems::new("%Y-%m-%d %H:%M:%S").collect();
    let specifics = RefreshKind::new()
        .with_cpu(CpuRefreshKind::new().with_cpu_usage())
        .with_memory(MemoryRefreshKind::everything());
    let system = RefCell::new(System::new_with_specifics(specifics));
    let update_status = move || {
        let battery = check_battery().unwrap();
        let volume = get_wireplumber();
        let brightness = get_brightness();
        let mut system = system.borrow_mut();
        system.refresh_specifics(specifics);
        let cpu_usage = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / 8.0;
        let used = system.used_memory() as f64 / (1024 * 1024) as f64;
        let total = system.total_memory() as f64 / (1024 * 1024) as f64;
        let s = Time::format_args(minute, Abbreviation);
        let battery_time_left = battery.time_to_empty();
        let battery_percent_left = battery.time_to_full();
        let battery_state_of_charge = battery.state_of_charge();
        let status = format!(
            r##"Brightness: {:.2} | {} | Battery: {:?}:{:?}-{:?}  <span color="#333333">|</span> MEM: {:.1}/{:.1} <span color="#333333">|</span> CPU: {:5.2} <span color="#333333">|</span> {}"##,
            brightness,
            volume.trim(),
            battery_time_left,
            battery_percent_left,
            battery_state_of_charge,
            used,
            total,
            cpu_usage,
            Local::now().format_with_items(time_format.iter())
        );
        set_status(&status);
    };
    update_status();
    let period = Duration::from_millis(300);
    let timer = get_timer("status_timer");
    timer.repeated(duration_until_wall_clock_is_multiple_of(period), period);
    timer.on_tick(update_status);
    Ok(())
}

pub fn configure() {
    // Configure seats and input devices
    let seat = get_seat("default");
    configure_seat(seat);
    let handle_input_device = move |device: InputDevice| {
        if device.has_capability(CAP_TOUCH) {
            device.set_left_handed(false);
            device.set_accel_profile(ACCEL_PROFILE_FLAT);
            device.set_accel_speed(1.70);
            device.set_transform_matrix([[0.35, 0.0], [0.0, 0.35]]);
        }
        device.set_natural_scrolling_enabled(true);
        device.set_tap_enabled(true);
        device.set_seat(seat);
    };
    input_devices().into_iter().for_each(handle_input_device);
    on_new_input_device(handle_input_device);

    set_env("GTK_THEME", "Adwaita:dark");
    set_env("XCURSOR_THEME", "Adwaita");
    // set_env("MESA_LOADER_DRIVER_OVERRIDE", "zink");

    // Configure the status message
    setup_status().expect("Status not working");

    // Start programs
    on_graphics_initialized(|| {
        Command::new("/usr/libexec/polkit-kde-authentication-agent-1").spawn();
        Command::new("fnott").spawn();
        Command::new("wbg")
            .arg("/home/uncomfy/.config/river/backgrounds/openSUSE-Gruvbox.png")
            .spawn();

        Command::new("jay")
            .arg("run-privileged")
            .arg("--")
            .arg("wl-paste")
            .arg("-t")
            .arg("text")
            .arg("--watch")
            .arg("clipman")
            .arg("store")
            .spawn();
    });

    on_idle(|| {
        Command::new("jay")
            .arg("run-privileged")
            .arg("--")
            .arg("waylock")
            .arg("-fork-on-lock")
            .spawn()
    })
}

config!(configure);
