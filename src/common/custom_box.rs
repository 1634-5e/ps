use crate::app::message::Message;
use iced::{Column, Element, Length, Row};

pub fn center_column<'a, E>(content: E) -> Column<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    Column::new()
        .push(Row::new().height(Length::FillPortion(1)))
        .push(content)
        .push(Row::new().height(Length::FillPortion(1)))
}

pub fn end_row<'a, E>(content: E) -> Row<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    Row::new()
        .push(Row::new().width(Length::FillPortion(1)))
        .push(content)
}
