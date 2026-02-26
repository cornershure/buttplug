// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2026 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
use uuid::Uuid;

use crate::device::{
  hardware::{HardwareCommand, HardwareWriteCmd},
  protocol::{ProtocolHandler, generic_protocol_setup},
};
use buttplug_core::errors::ButtplugDeviceError;
use buttplug_server_device_config::Endpoint;

generic_protocol_setup!(TCodeV03, "tcode-v03");

#[derive(Default)]
pub struct TCodeV03 {}

impl ProtocolHandler for TCodeV03 {

    // Axis definitions
    // These are the axes we are working with, mapped to their corresponding 'L' (Linear) and 'R' (Rotational) channels.
    // Note: Each axis maps to an ID and description, e.g., 'L0' (Stroke), 'L1' (Forward), etc.
    const AXES: [&str; 6] = ["L0", "L1", "L2", "R0", "R1", "R2"];

    fn handle_output_position_cmd(
        &self,
        _feature_index: u32,
        feature_id: Uuid,
        positions: Vec<u32>,  // Now handling multiple axes with a Vec
    ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
        let mut msg_vec = vec![];

        // Generate the command string for each axis
        let mut command = String::new();
        for (i, &position) in positions.iter().enumerate() {
            if i < 3 {
                // Linear axes L0, L1, L2
                command.push_str(&format!("L{i}{position:03}\n"));
            } else {
                // Rotational axes R0, R1, R2
                command.push_str(&format!("R{i - 3}{position:03}\n"));
            }
        }

        // Send the command to the hardware
        msg_vec.push(
            HardwareWriteCmd::new(
                &[feature_id],
                Endpoint::Tx,
                command.as_bytes().to_vec(),
                false,
            )
            .into(),
        );

        Ok(msg_vec)
    }

    fn handle_hw_position_with_duration_cmd(
        &self,
        feature_index: u32,
        feature_id: Uuid,
        positions: Vec<u32>,  // Accepts multiple positions for multi-axis
        durations: Vec<u32>,  // Duration for each axis
    ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
        let mut msg_vec = vec![];

        // Generate the command string for each axis with duration
        let mut command = String::new();
        for (i, (&position, &duration)) in positions.iter().zip(durations.iter()).enumerate() {
            if i < 3 {
                // Linear axes L0, L1, L2
                command.push_str(&format!("L{i}{position:03}I{duration}\n"));
            } else {
                // Rotational axes R0, R1, R2
                command.push_str(&format!("R{i - 3}{position:03}I{duration}\n"));
            }
        }

        // Send the command to the hardware
        msg_vec.push(
            HardwareWriteCmd::new(
                &[feature_id],
                Endpoint::Tx,
                command.as_bytes().to_vec(),
                false,
            )
            .into(),
        );

        Ok(msg_vec)
    }

    fn handle_output_vibrate_cmd(
        &self,
        feature_index: u32,
        feature_id: Uuid,
        speed: u32,
    ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
        Ok(vec![
            HardwareWriteCmd::new(
                &[feature_id],
                Endpoint::Tx,
                format!("V{feature_index}{speed:02}\n").as_bytes().to_vec(),
                false,
            )
            .into(),
        ])
    }
}
