use std::fs;

use iced::{Element, Length, Padding, Size, Task, alignment, application, widget::{Button, Image, button, column, container, image, row, text}};
use iced::widget::image::Handle;
use iced_aw::{menu::{self, Menu}, ICED_AW_FONT_BYTES, menu_bar, menu_items};


mod repository;
mod utils;
mod views;

use views::preset_info::PresetInfoView;
use repository::fake_db_interface::{Ticket, TicketPreset};
use utils::ticket;

use crate::utils::convert::convert_vec_to_str;


fn main() -> iced::Result {
    application(App::default, App::update, App::view)
        .title("Lottery Ticket Generator")
        .font(ICED_AW_FONT_BYTES)
        .window_size(Size::new(500.0, 800.0))
        .run()
}

#[derive(Clone)]
enum Message {
    NoOp,
    PresetInfoSelected,
    TicketGenSelected,
    ShowPresets,
    CheckTicketHistory,
    GenBtnPressed,
}

enum ViewId {
    MainPage,
    TicektGen,
    TicketHistory,
    Presets,
    PresetInfo
}

struct View {
    active_view: ViewId,
    preset_info: PresetInfoView,
}

struct App {
    default_preset_name: String,
    saved_tickets: Vec<Ticket>,
    saved_presets: Vec<TicketPreset>,
    generated_tickets: Vec<Ticket>,
    new_preset: TicketPreset,
    new_preset_nums_generated: u32,
    demo_ticket_val: Vec<i32>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            default_preset_name: String::new(),
            saved_tickets: Vec::new(),
            saved_presets: Vec::new(),
            generated_tickets: Vec::new(),
            new_preset: TicketPreset::new(),
            new_preset_nums_generated: 0,
            demo_ticket_val: Vec::new(),
        };
        app
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => {
                Task::none()
            }
            Message::PresetInfoSelected => {
                Task::none()
            }
            Message::GenBtnPressed => {
                let demo_limits : Vec<i32> = vec![70, 70, 70, 70, 70, 65];
                self.demo_ticket_val = ticket::generate(&demo_limits);
                Task::none()
            }
            _ => Task::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        
        
        let file_menu = Menu::new(menu_items!(
            (
                button(container(text("Create Preset")).width(Length::Fill))
                    .on_press(Message::NoOp)
                    .style(button::text)
                    .width(Length::Fill)
            ),
            (
                button(container(text("Create Ticket(s)")).width(Length::Fill))
                    .on_press(Message::NoOp)
                    .style(button::text)
                    .width(Length::Fill)
            ),
            (
                button(container(text("Ticket History")).width(Length::Fill))
                    .on_press(Message::NoOp)
                    .style(button::text)
                    .width(Length::Fill)
            ),
        ))
        .width(200)
        .close_on_item_click(true);

        let edit_menu = Menu::new(menu_items!(
            (
                button(container(text("Default Preset")).width(Length::Fill))
                    .on_press(Message::NoOp)
                    .style(button::text)
                    .width(Length::Fill)
            ),
            (
                button(container(text("Edit Preset")).width(Length::Fill))
                    .on_press(Message::NoOp)
                    .style(button::text)
                    .width(Length::Fill)
            ),
        ))
        .width(200)
        .close_on_item_click(true);
        
        
        let menu_bar = menu_bar!(
            (container(text("File")), file_menu),
            (container(text("Edit")), edit_menu)
        )
        .width(Length::Fill)
        .spacing(7)
        .padding(Padding::new(7.0))
        .draw_path(menu::DrawPath::Backdrop)
        .close_on_background_click_global(true);

        // let image_bytes = fs::read("../assets/burning_money.png").expect("Failed to load intro page image");
        // let image_handler = <iced::widget::Image<_> as Example>::Handle::from_bytes(image_bytes);
        let start_image: iced::widget::Image<Handle>  = image("src/assets/burning_money.jpg")
            .width(400)
            .height(400);

        let start_btn = button("Create Tickets")
            .on_press(Message::GenBtnPressed)
            .padding(20);

        let demo_text = if self.demo_ticket_val.is_empty() {
            text("Click button above to generate a ticket")
        } else {
            let ticket_str = convert_vec_to_str(self.demo_ticket_val.clone());
            text(ticket_str)
        };

        let content = column![
            menu_bar,
            container(start_image).width(Length::Fill).align_x(alignment::Alignment::Center),
            container(start_btn).width(Length::Fill).align_x(alignment::Alignment::Center),
            container(demo_text).width(Length::Fill).align_x(alignment::Alignment::Center),
        ]
        .spacing(10);

        content.into()
    }
}