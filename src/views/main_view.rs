use iced::{
    alignment,
    widget::{button, column, container, row, text, text_editor, text_input},
    Element, Length,
};

use crate::messages::Message;
use crate::styles::{terminal_style, SecondaryButtonStyle};

pub fn main_view<'a>(
    ip_address: &str,
    log_editor_content: &'a text_editor::Content,
    is_running: bool,
) -> Element<'a, Message> {
    let header = row![
        text("Manual Device Entry")
            .size(24)
            .width(Length::Fill),
        button(text("Back").size(14))
            .on_press(Message::CloseManualEntry)
            .padding([8, 16])
            .style(iced::theme::Button::custom(SecondaryButtonStyle))
    ]
    .align_items(alignment::Alignment::Center)
    .spacing(10);

    let ip_label = text("Device IP Address").size(14);

    let ip_input = text_input("192.168.1.20", ip_address)
        .on_input(Message::IpAddressChanged)
        .padding(12)
        .size(16);

    let adopt_button = if is_running {
        button(
            text("Adopting...")
                .size(16)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding([12, 24])
        .width(Length::Fill)
    } else {
        button(
            text("Adopt Default (ubnt)")
                .size(16)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(Message::AdoptClicked)
        .padding([12, 24])
        .width(Length::Fill)
    };

    let adopt_again_button = if is_running {
        button(
            text("Adopting...")
                .size(16)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding([12, 24])
        .width(Length::Fill)
    } else {
        button(
            text("Adopt (alternate credentials)")
                .size(16)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(Message::AdoptAgainClicked)
        .padding([12, 24])
        .width(Length::Fill)
    };

    let logs_label = text("Console Output").size(14);

    let log_content = text_editor(log_editor_content)
        .height(Length::Fill)
        .font(iced::Font::MONOSPACE)
        .on_action(Message::LogEditorAction);

    let logs_box = container(log_content)
        .style(terminal_style)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(1);

    let content = column![
        header,
        container(
            column![
                ip_label,
                ip_input,
                adopt_button,
                adopt_again_button,
            ]
            .spacing(8)
        ).padding([10, 0]),
        container(
            column![
                logs_label,
                logs_box,
            ]
            .spacing(8)
        ).padding([10, 0]),
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
