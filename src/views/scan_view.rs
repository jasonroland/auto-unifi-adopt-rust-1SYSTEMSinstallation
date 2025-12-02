use iced::{
    alignment,
    widget::{button, checkbox, column, container, mouse_area, row, scrollable, text, text_input},
    Background, Color, Element, Length, Theme,
};

use crate::messages::Message;
use crate::models::{Device, DeviceStatus};
use crate::styles::{terminal_style, SecondaryButtonStyle};

pub fn scan_view(
    ip_range_start: &str,
    ip_range_end: &str,
    devices: &[Device],
    expanded_device_index: Option<usize>,
    is_scanning: bool,
) -> Element<'static, Message> {
    let title = row![
        text("Network Scanner")
            .size(24)
            .width(Length::Fill),
        button(text("Manual Entry").size(14))
            .on_press(Message::ManualEntryClicked)
            .padding([8, 16])
            .style(iced::theme::Button::custom(SecondaryButtonStyle)),
        button(text("Settings").size(14))
            .on_press(Message::SettingsClicked)
            .padding([8, 16])
            .style(iced::theme::Button::custom(SecondaryButtonStyle))
    ]
    .align_items(alignment::Alignment::Center)
    .spacing(10);

    let scan_button = if is_scanning {
        button(
            row![
                text("⟳").size(14),
                text(" Scanning...").size(14)
            ]
            .spacing(5)
            .align_items(alignment::Alignment::Center)
        )
        .padding([10, 20])
    } else {
        button(text("Scan").size(14))
            .on_press(Message::ScanDevices)
            .padding([10, 20])
    };

    let ip_range_section = column![
        text("IP Range").size(14),
        row![
            text_input("192.168.1.1", ip_range_start)
                .on_input(Message::IpRangeStartChanged)
                .padding(10)
                .size(14),
            text_input("192.168.1.254", ip_range_end)
                .on_input(Message::IpRangeEndChanged)
                .padding(10)
                .size(14),
            scan_button
        ]
        .spacing(10)
        .align_items(alignment::Alignment::Center),
    ]
    .spacing(6);

    let device_list_section = if is_scanning {
        build_scanning_view()
    } else {
        build_full_view(devices, expanded_device_index)
    };

    let adopt_buttons = row![
        button(text("Adopt Default (ubnt)").size(14).horizontal_alignment(alignment::Horizontal::Center))
            .on_press(Message::AdoptSelectedDefault)
            .padding([10, 20])
            .width(Length::Fill),
        button(text("Adopt (alternate credentials)").size(14).horizontal_alignment(alignment::Horizontal::Center))
            .on_press(Message::AdoptSelectedAlt)
            .padding([10, 20])
            .width(Length::Fill),
    ]
    .spacing(10);

    let content = column![
        title,
        ip_range_section,
        container(device_list_section).padding([10, 0]),
        adopt_buttons,
    ]
    .spacing(15)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn build_full_view(devices: &[Device], expanded_device_index: Option<usize>) -> Element<'static, Message> {
    let table_header = container(
        row![
            container(text("")).width(Length::Fixed(40.0)),
            container(text("IP Address").size(13)).width(Length::FillPortion(2)),
            container(text("MAC Address").size(13)).width(Length::FillPortion(2)),
            container(text("Company").size(13)).width(Length::FillPortion(3)),
            container(text("Status").size(13)).width(Length::Fixed(60.0)),
        ]
        .padding(10)
        .spacing(10)
    )
    .padding([0, 1])
    .width(Length::Fill);

    let device_rows = build_grouped_device_rows(devices, expanded_device_index, true);

    let table_content = scrollable(
        column![table_header, device_rows]
    )
    .height(Length::Fill);

    let table_box = container(table_content)
        .style(terminal_style)
        .width(Length::Fill)
        .height(Length::Fill);

    container(
        column![
            text("Discovered Devices").size(14),
            table_box,
        ]
        .spacing(8)
    )
    .height(Length::Fill)
    .into()
}

fn build_grouped_device_rows(
    devices: &[Device],
    expanded_device_index: Option<usize>,
    show_company: bool,
) -> Element<'static, Message> {
    let mut device_rows = column![].spacing(0);

    // Separate devices into SSH and non-SSH groups
    let ssh_devices: Vec<(usize, &Device)> = devices
        .iter()
        .enumerate()
        .filter(|(_, d)| d.has_ssh)
        .collect();

    let non_ssh_devices: Vec<(usize, &Device)> = devices
        .iter()
        .enumerate()
        .filter(|(_, d)| !d.has_ssh)
        .collect();

    // Add SSH devices section
    if !ssh_devices.is_empty() {
        let ssh_header = container(
            container(
                text("SSH Enabled")
                    .size(12)
                    .style(Color::from_rgb(0.4, 0.4, 0.4))
            )
            .padding([8, 15])
            .width(Length::Fill)
            .style(|_theme: &Theme| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.85, 0.95, 0.85))),
                ..Default::default()
            })
        )
        .padding([0, 1]);
        device_rows = device_rows.push(ssh_header);

        for (row_num, (index, device)) in ssh_devices.iter().enumerate() {
            let device_row = build_device_row(
                *index,
                device,
                expanded_device_index,
                show_company,
                row_num,
            );
            device_rows = device_rows.push(device_row);
        }
    }

    // Add non-SSH devices section
    if !non_ssh_devices.is_empty() {
        let non_ssh_header = container(
            container(
                text("SSH Disabled")
                    .size(12)
                    .style(Color::from_rgb(0.4, 0.4, 0.4))
            )
            .padding([8, 15])
            .width(Length::Fill)
            .style(|_theme: &Theme| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.85, 0.85))),
                ..Default::default()
            })
        )
        .padding([0, 1]);
        device_rows = device_rows.push(non_ssh_header);

        for (row_num, (index, device)) in non_ssh_devices.iter().enumerate() {
            let device_row = build_device_row(
                *index,
                device,
                expanded_device_index,
                show_company,
                row_num,
            );
            device_rows = device_rows.push(device_row);
        }
    }

    device_rows.into()
}

fn build_device_row(
    index: usize,
    device: &Device,
    expanded_device_index: Option<usize>,
    show_company: bool,
    row_num: usize,
) -> Element<'static, Message> {
    let is_expanded = Some(index) == expanded_device_index;
    let row_bg = if is_expanded {
        Color::from_rgb(0.85, 0.90, 0.95)
    } else if row_num % 2 == 0 {
        Color::from_rgb(0.99, 0.99, 0.99)
    } else {
        Color::from_rgb(0.96, 0.96, 0.96)
    };

    let status_icon = build_status_icon(&device.status);

    let mut row_content = row![
        container(
            checkbox("", device.selected)
                .on_toggle(move |checked| Message::DeviceToggled(index, checked))
        )
        .width(Length::Fixed(40.0))
        .center_x(),
        container(text(&device.ip).size(13)).width(Length::FillPortion(2)),
        container(text(&device.mac).size(13)).width(Length::FillPortion(2)),
    ]
    .spacing(10)
    .padding(8);

    if show_company {
        let company_text = if device.company.len() > 30 {
            format!("{}...", &device.company[..27])
        } else {
            device.company.clone()
        };
        row_content = row_content.push(
            container(text(company_text).size(13)).width(Length::FillPortion(3))
        );
    }

    row_content = row_content.push(
        container(status_icon)
            .width(Length::Fixed(60.0))
            .center_x()
    );

    let main_row = container(
        mouse_area(
            container(row_content)
                .style(move |_theme: &Theme| container::Appearance {
                    background: Some(Background::Color(row_bg)),
                    ..Default::default()
                })
                .width(Length::Fill)
        )
        .on_press(Message::DeviceRowClicked(index))
    )
    .padding([0, 1])
    .width(Length::Fill);

    // If expanded and not pending (including in progress), add the logs section below
    if is_expanded && device.status != DeviceStatus::Pending {
        let logs_box = container(
            container(
                text(&device.logs)
                    .size(13)
                    .font(iced::Font::MONOSPACE)
            )
            .padding(12)
            .width(Length::Fill)
        )
        .style(terminal_style)
        .width(Length::Fill);

        let logs_section = container(
            column![
                text(format!("Logs for {}", device.ip)).size(14),
                logs_box,
            ]
            .spacing(8)
        )
        .padding([8, 24, 8, 24])
        .style(move |_theme: &Theme| container::Appearance {
            background: Some(Background::Color(row_bg)),
            ..Default::default()
        })
        .width(Length::Fill);

        column![main_row, logs_section]
            .spacing(0)
            .into()
    } else {
        main_row.into()
    }
}

fn build_status_icon(status: &DeviceStatus) -> Element<'static, Message> {
    match status {
        DeviceStatus::Pending => {
            text("").into()
        }
        DeviceStatus::InProgress => {
            text("⟳")
                .size(18)
                .style(Color::from_rgb(0.0, 0.5, 0.8))
                .into()
        }
        DeviceStatus::Success => {
            text("✓")
                .size(18)
                .style(Color::from_rgb(0.0, 0.6, 0.0))
                .into()
        }
        DeviceStatus::Error => {
            text("⚠")
                .size(18)
                .style(Color::from_rgb(0.8, 0.2, 0.0))
                .into()
        }
    }
}

fn build_scanning_view() -> Element<'static, Message> {
    let loader = column![
        text("⟳").size(48).style(Color::from_rgb(0.4, 0.6, 0.8)),
        text("Scanning network...").size(16).style(Color::from_rgb(0.5, 0.5, 0.5)),
    ]
    .spacing(15)
    .align_items(alignment::Alignment::Center);

    container(
        column![
            text("Discovered Devices").size(14),
            container(
                container(loader)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            )
            .style(terminal_style)
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .spacing(8)
    )
    .height(Length::Fill)
    .into()
}
