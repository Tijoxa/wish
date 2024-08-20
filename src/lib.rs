use iced::{
    alignment,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Element, Length, Padding, Sandbox, Theme,
};

pub struct Wish {
    items: Vec<String>,
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputValue(String),
    Submitted,
    DeleteItem(usize),
}

impl Sandbox for Wish {
    type Message = Message;

    fn new() -> Self {
        Self {
            items: vec![
                "Primogem".into(),
                "Asterie".into(),
                "Stone".into(),
                "Asterion".into(),
                "Cristal".into(),
            ],
            input_value: String::default(),
        }
    }

    fn title(&self) -> String {
        "A cool application".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                self.items.push(self.input_value.clone());
                self.input_value = String::default();
            }
            Message::DeleteItem(item) => {
                self.items.remove(item);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        container(
            column!(
                items_list_view(&self.items),
                row!(
                    text_input("Input resource", &self.input_value)
                        .on_input(|value| Message::InputValue(value.to_lowercase()))
                        .on_submit(Message::Submitted),
                    button("Submit").on_press(Message::Submitted)
                )
                .spacing(30)
                .padding(Padding::from(30))
            )
            .align_items(iced::Alignment::Center),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);

    for (index, value) in items.iter().enumerate() {
        let to_push: Element<'static, Message> = row!(
            text(value),
            button("Delete").on_press(Message::DeleteItem(index))
        )
        .align_items(iced::Alignment::Center)
        .spacing(30)
        .into();
        column = column.push(to_push);
    }

    scrollable(container(column))
        .height(250.0)
        .width(300)
        .into()
}
