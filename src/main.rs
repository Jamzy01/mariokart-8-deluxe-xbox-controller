use rand::Rng;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use winput::{Button, Vk};

use gamepad_rs::*;

pub fn main() {
    let mut controller = ControllerContext::new().unwrap();
    let mut pressed_keys: Vec<Vk> = Vec::new();

    let mut connected_device: Option<String> = None;
    let mut acceleration: f32 = 0.;
    let mut steering: f32 = 0.;

    loop {
        // Get Time in MS

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let time = since_the_epoch.as_millis();

        // Find Controllers

        if controller.scan_controllers() < 1 {
            if connected_device.is_some() {
                println!("{} Was Disconnected", connected_device.unwrap());

                connected_device = None;

                // Unpress all keys

                for pressed_key in pressed_keys.iter() {
                    winput::release(pressed_key.clone());
                }

                pressed_keys = Vec::new();
            }

            continue; // Failed to find a controller
        }

        // Update Controller

        controller.update(0);

        // Get The State Of The Controller

        let status = controller.state(0).status;

        if status == ControllerStatus::Connected {
            let nb_buttons;
            let nb_axis;
            {
                let info = controller.info(0);
                nb_buttons = info.digital_count;
                nb_axis = info.analog_count;

                if connected_device != Some(info.name.to_string()) {
                    println!(
                        "Connected To Device {} with {} buttons and {} axis",
                        info.name, info.digital_count, info.analog_count
                    );
                }

                connected_device = Some(info.name.to_string());
            }
            {
                let state = controller.state(0);

                // Get Old Pressed Keys

                let old_pressed_keys = pressed_keys.clone();

                pressed_keys = Vec::new();

                // Acceleration

                let input_acceleration =
                    (state.analog_state[3] + 1.) / 2. - (state.analog_state[2] + 1.) / 2.;

                acceleration += (input_acceleration - acceleration) / 1.6;

                if acceleration > 0.1 {
                    pressed_keys.push(Vk::Z);
                }

                if acceleration < -0.1 {
                    pressed_keys.push(Vk::X);
                }

                // Steering

                let input_steering = state.analog_state[0];

                steering += (input_steering - steering) / 2.;

                if steering > 0.1 {
                    if rand::thread_rng().gen_range(0..1000)
                        <= (f32::powf(steering, 2.5) * 1000.) as u128
                    {
                        pressed_keys.push(Vk::D);
                    }
                }

                if steering < -0.1 {
                    if rand::thread_rng().gen_range(0..1000)
                        <= (f32::powf(-steering, 2.5) * 1000.) as u128
                    {
                        pressed_keys.push(Vk::A);
                    }
                }

                // Drifting

                if state.digital_state[3] || state.digital_state[13] {
                    // 13 is rb which is only used to start the game when it boots up
                    pressed_keys.push(Vk::Space);
                }

                // Items

                if state.digital_state[2] || state.digital_state[12] {
                    // 12 is lb which is only used to start the game when it boots up
                    pressed_keys.push(Vk::E);
                }

                // UI

                if state.digital_state[0] {
                    pressed_keys.push(Vk::Z);
                }

                if state.digital_state[1] {
                    pressed_keys.push(Vk::X);
                }

                if state.analog_state[4] > 0.7 || state.digital_state[7] {
                    pressed_keys.push(Vk::D);
                }

                if state.analog_state[4] < -0.7 || state.digital_state[6] {
                    pressed_keys.push(Vk::A);
                }

                // UI and Glider Control

                if (state.analog_state[5] + state.analog_state[1]) > 0.7 || state.digital_state[4] {
                    pressed_keys.push(Vk::W);
                }

                if (state.analog_state[5] + state.analog_state[1]) < -0.7 || state.digital_state[5]
                {
                    pressed_keys.push(Vk::S);
                }

                for pressed_key in old_pressed_keys.iter() {
                    if !pressed_keys.contains(pressed_key) {
                        winput::release(pressed_key.clone());
                    }
                }

                for pressed_key in pressed_keys.iter() {
                    winput::press(pressed_key.clone());
                }
            }
        } else {
            if connected_device.is_some() {
                println!("{} Was Disconnected", connected_device.unwrap());

                connected_device = None;

                // Unpress all keys

                for pressed_key in pressed_keys.iter() {
                    winput::release(pressed_key.clone());
                }

                pressed_keys = Vec::new();
            }

            continue;
        }

        thread::sleep(Duration::from_millis(5));
    }
}
