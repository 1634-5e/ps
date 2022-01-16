use crate::app::message::Message;
use iced::{Column, Element, Length, Row};

//让元素出于一行的1/n的位置
//FIXME: 优化算法
pub fn row_with_blanks<'a, E>(content: E, before: usize, after: usize) -> Row<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    let mut res = Row::new();
    for _ in 0..before {
        res = res.push(Row::new().width(Length::FillPortion(1)));
    }
    res = res.push(content);
    for _ in 0..after {
        res = res.push(Row::new().width(Length::FillPortion(1)));
    }
    res
}

pub fn column_with_blanks<'a, E>(content: E, before: usize, after: usize) -> Column<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    let mut res = Column::new();
    for _ in 0..before {
        res = res.push(Column::new().width(Length::FillPortion(1)));
    }
    res = res.push(content);
    for _ in 0..after {
        res = res.push(Column::new().width(Length::FillPortion(1)));
    }
    res
}