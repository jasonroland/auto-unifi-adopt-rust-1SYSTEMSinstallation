use iced::{
    alignment,
    widget::{button, column, container, row, text, text_input},
    Background, Border, Color, Element, Length, Theme,
};

use crate::messages::Message;
use crate::models::SettingsTab;
use crate::styles::SecondaryButtonStyle;

pub fn settings_view(
    active_tab: &SettingsTab,
    controller_url_input: &str,
    username_input: &str,
    password_input: &str,
    alt_username_input: &str,
    alt_password_input: &str,
) -> Element<'static, Message> {
    let title = text("Settings").size(24);

    let general_label = if *active_tab == SettingsTab::General {
        format!("→ General")
    } else {
        "General".to_string()
    };

    let general_tab = button(
        text(general_label)
            .size(14)
            .horizontal_alignment(alignment::Horizontal::Center)
    )
    .on_press(Message::TabSelected(SettingsTab::General))
    .padding([10, 20])
    .width(Length::Fill);

    let default_label = if *active_tab == SettingsTab::DefaultCredentials {
        format!("→ Default Credentials")
    } else {
        "Default Credentials".to_string()
    };

    let default_creds_tab = button(
        text(default_label)
            .size(14)
            .horizontal_alignment(alignment::Horizontal::Center)
    )
    .on_press(Message::TabSelected(SettingsTab::DefaultCredentials))
    .padding([10, 20])
    .width(Length::Fill);

    let alt_label = if *active_tab == SettingsTab::AlternativeCredentials {
        format!("→ Alternative Credentials")
    } else {
        "Alternative Credentials".to_string()
    };

    let alt_creds_tab = button(
        text(alt_label)
            .size(14)
            .horizontal_alignment(alignment::Horizontal::Center)
    )
    .on_press(Message::TabSelected(SettingsTab::AlternativeCredentials))
    .padding([10, 20])
    .width(Length::Fill);

    let tabs = row![general_tab, default_creds_tab, alt_creds_tab]
        .spacing(5)
        .padding([0, 0, 15, 0]);

    let tab_content = match active_tab {
        SettingsTab::General => {
            column![
                text("Controller Settings").size(16),
                column![
                    text("Controller URL").size(13),
                    text_input("http://192.168.1.1:8080", controller_url_input)
                        .on_input(Message::ControllerUrlChanged)
                        .padding(10)
                        .size(14),
                ]
                .spacing(6),
            ]
            .spacing(15)
        }
        SettingsTab::DefaultCredentials => {
            column![
                text("Default SSH Credentials (ubnt)").size(16),
                text("These credentials are used with the 'Adopt Default (ubnt)' button")
                    .size(12),
                column![
                    text("SSH Username").size(13),
                    text_input("ubnt", username_input)
                        .on_input(Message::UsernameChanged)
                        .padding(10)
                        .size(14),
                ]
                .spacing(6),
                column![
                    text("SSH Password").size(13),
                    text_input("ubnt", password_input)
                        .on_input(Message::PasswordChanged)
                        .padding(10)
                        .size(14),
                ]
                .spacing(6),
            ]
            .spacing(15)
        }
        SettingsTab::AlternativeCredentials => {
            column![
                text("Alternative SSH Credentials").size(16),
                text("These credentials are used with the 'Adopt Again' button for non-default setups")
                    .size(12),
                column![
                    text("SSH Username").size(13),
                    text_input("admin", alt_username_input)
                        .on_input(Message::AltUsernameChanged)
                        .padding(10)
                        .size(14),
                ]
                .spacing(6),
                column![
                    text("SSH Password").size(13),
                    text_input("password", alt_password_input)
                        .on_input(Message::AltPasswordChanged)
                        .padding(10)
                        .size(14),
                ]
                .spacing(6),
            ]
            .spacing(15)
        }
    };

    let buttons = row![
        button(text("Back").size(14))
            .on_press(Message::CloseSettings)
            .padding([10, 20])
            .style(iced::theme::Button::custom(SecondaryButtonStyle)),
        button(text("Save Settings").size(14))
            .on_press(Message::SaveSettings)
            .padding([10, 20]),
    ]
    .spacing(10);

    let content = column![
        title,
        tabs,
        container(tab_content)
            .padding(20)
            .style(|_theme: &Theme| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                border: Border {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fixed(260.0)),
        buttons,
    ]
    .spacing(15)
    .padding(25)
    .max_width(600);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
