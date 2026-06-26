use iced::{Element, Length};
use iced::widget::{Button, Scrollable, TextInput, column, container, row, text};

use iced_aw::NumberInput;

use crate::repository::fake_db_interface::{TicketPreset};

use crate::{Message, View};

struct NumberBounds{
    upper: i32,
    lower: i32
}



#[derive(Debug, Clone)]
pub struct PresetInfoView {}

enum PresetInfoMessage {

}

impl PresetInfoView {
    fn update(&mut self, message: PresetInfoMessage){

    }

    fn view(&self) -> Element<'_, Message> {
        let mut new_preset = TicketPreset::new();

        let name_row = row![
                text("Name: "),
                TextInput::new("", &new_preset.name)
                    // .on_input(Message::NoOp)
                    .width(Length::Fill)
            ];

            let generated_num_size: u32 = 0;
            let mut num_str = "";
            let value_row = row![
                text("# of vals: "),
                row![
                    TextInput::new("", num_str)
                        .width(Length::Fill),
                    // NumberInput::new(generated_num_size, bounds, on_change)
                ],
            ];

            let mut val_limits = column![];
            let mut limits_vec: Vec<NumberBounds> = Vec::new();
            let size: usize = generated_num_size as usize;
            for i in 0..size {
                let lower_limit_str = "";
                let upper_limit_str = "";
                limits_vec.push(NumberBounds { upper: 0, lower: 0 });
                val_limits = val_limits.push(
                    container(
                        row![
                            text(format!("Num {}:", i)),
                            TextInput::new("", lower_limit_str),
                            text(" - "),
                            TextInput::new("", upper_limit_str)
                        ]
                    )
            );
        }

        let content = column![
            name_row,
            value_row,
            val_limits,
            row![
                Button::new("Save"),
                Button::new("Close")
            ]
        ];

        container(content).width(Length::Fill).center_x(Length::Fill).into()
    }
}