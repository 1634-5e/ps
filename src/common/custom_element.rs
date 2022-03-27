use crate::app::message::MessageType;
use iced::{Column, Element, Length, Row};

//让元素出于一行的1/n的位置
//FIXME: 优化算法
//这里涉及了两个generic type的使用，还有lifetime，可能会有bug
pub fn row_with_blanks<'a, E, T: 'a>(content: E, before: usize, after: usize) -> Row<'a, T>
where
    E: Into<Element<'a, T>>,
    T: MessageType,
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

    //这样写虽然看起来简洁了，但是没用，不太明白
    // Row::new()
    //     .push(Row::new().width(Length::FillPortion(before)))
    //     .push(content)
    //     .push(Row::new().width(Length::FillPortion(after)))
}

pub fn column_with_blanks<'a, E, T: 'a>(content: E, before: usize, after: usize) -> Column<'a, T>
where
    E: Into<Element<'a, T>>,
    T: MessageType,
{
    let mut res = Column::new();
    for _ in 0..before {
        res = res.push(Column::new().height(Length::FillPortion(1)));
    }
    res = res.push(content);
    for _ in 0..after {
        res = res.push(Column::new().height(Length::FillPortion(1)));
    }
    res
}
