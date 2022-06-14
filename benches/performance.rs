use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use iced::keyboard::{KeyCode, Modifiers};
use iced::mouse::ScrollDelta;
use iced::{window, Point, Settings};
// use iced::time::every;
use iced::pure::Application;
use iced_native::mouse::Event as MouseEvent;
use iced_native::window::Event as WindowEvent;
use iced_native::Event;

use ps::ui::*;
use ps::*;

fn criterion_benchmark(c: &mut Criterion) {
    //empty startup
    c.bench_function("empty startup", |b| {
        b.iter(|| {
            Ps::run(black_box(Settings {
                flags: Flags {
                    env_args: vec![],
                    // user_settings,
                },
                antialiasing: true,
                window: window::Settings {
                    position: window::Position::Specific(0, 0),
                    size: (50, 50),
                    ..window::Settings::default()
                },
                ..Settings::default()
            }))
        })
    });

    //startup with files
    c.bench_function("start up with files", |b| {
        b.iter(|| {
            Ps::run(black_box(Settings {
                flags: Flags {
                    env_args: vec![
                        PathBuf::from(""),
                        PathBuf::from("D:\\photos\\Snipaste_2021-12-20_16-16-48.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2021-12-29_15-15-42.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2021-12-29_15-15-59.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2021-12-29_15-17-15.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2022-01-12_19-55-57.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2022-01-13_12-05-24.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2022-01-02_15-14-47.png"),
                        PathBuf::from("D:\\photos\\Snipaste_2022-01-13_12-05-39.png"),
                    ],
                    // user_settings,
                },
                antialiasing: true,
                window: window::Settings {
                    position: window::Position::Specific(0, 0),
                    size: (50, 50),
                    ..window::Settings::default()
                },
                ..Settings::default()
            }))
        })
    });

    let mut ps = Ps::Loaded(Box::new(State::default()));

    //拖拽到应用
    c.bench_function("drop file to program", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Window(
                WindowEvent::FileDropped(PathBuf::from(
                    "D:\\photos\\Snipaste_2021-12-20_16-16-48.png",
                )),
            )));
            ps.view();
        })
    });

    sleep(Duration::from_secs(5));

    //滚轮切换图片
    c.bench_function("scroll to navigage back", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Mouse(
                MouseEvent::WheelScrolled {
                    delta: ScrollDelta::Pixels { x: 0.0, y: 1.0 },
                },
            )));
            ps.view();
        })
    });

    sleep(Duration::from_secs(5));

    c.bench_function("scroll to navigate next", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Mouse(
                MouseEvent::WheelScrolled {
                    delta: ScrollDelta::Pixels { x: 0.0, y: -1.0 },
                },
            )));
            ps.view();
        })
    });

    //删除图片
    c.bench_function("delete image", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Keyboard(
                iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Delete,
                    modifiers: Modifiers::default(),
                },
            )));
            ps.view();
        })
    });

    //键盘上切换图片
    c.bench_function("↑切换", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Keyboard(
                iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Up,
                    modifiers: Modifiers::default(),
                },
            )));
            ps.view();
        })
    });
    //键盘左切换图片
    c.bench_function("←切换", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Keyboard(
                iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Left,
                    modifiers: Modifiers::default(),
                },
            )));
            ps.view();
        })
    });
    //键盘下切换图片
    c.bench_function("↓切换", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Keyboard(
                iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Down,
                    modifiers: Modifiers::default(),
                },
            )));
            ps.view();
        })
    });
    //键盘右切换图片
    c.bench_function("→切换", |b| {
        b.iter(|| {
            ps.update(Message::ExternEvent(Event::Keyboard(
                iced::keyboard::Event::KeyPressed {
                    key_code: KeyCode::Right,
                    modifiers: Modifiers::default(),
                },
            )));
            ps.view();
        })
    });

    //自动保存
    ps.view();
    c.bench_function("auto save", |b| {
        b.iter(|| {
            ps.update(Message::AutoSave);
            ps.view();
        })
    });

    //保存结果处理
    c.bench_function("perform after auto save", |b| {
        b.iter(|| ps.update(Message::SavedOrFailed(Ok(()))));
        ps.view();
    });

    //关闭未找到的图片
    c.bench_function("close not found", |b| {
        b.iter(|| ps.update(Message::Viewer(ViewerMessage::CloseNotFound)));
        ps.view();
    });

    //关闭图片
    c.bench_function("close image", |b| {
        b.iter(|| ps.update(Message::Toolbar(ToolbarMessage::Close)));
        ps.view();
    });

    //清空图片
    c.bench_function("clear image", |b| {
        b.iter(|| ps.update(Message::Viewer(ViewerMessage::JumpToImage(10))));
        ps.view();
    });

    if let Ps::Loaded(state) = &mut ps {
        state.edit.copied_curve = Some(Rc::new(RefCell::new(Curve {
            shape: Rectangle::default().into(),
            ..Curve::default()
        })));
        ps.view();
    }

    //创建直线
    c.bench_function("create line", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 100.0 }),
            ))));
            ps.view();
        })
    });

    //选择矩形
    c.bench_function("select shape:rect", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::ChangeShape(
                Rectangle::default().into(),
            )));
            ps.view();
        })
    });

    //创建矩形
    c.bench_function("create rect", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 100.0 }),
            ))));
            ps.view();
        })
    });

    //选择三角形
    c.bench_function("select shape:tria", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::ChangeShape(
                Triangle::default().into(),
            )));
            ps.view();
        })
    });

    //创建三角形
    c.bench_function("create triangle", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 1.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 10.0 }),
            ))));
            ps.view();
        })
    });

    //选择贝塞尔
    c.bench_function("select bezier", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::ChangeShape(
                QuadraticBezier::default().into(),
            )));
            ps.view();
        })
    });

    //创建贝塞尔
    c.bench_function("create bezier", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 1.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 10.0 }),
            ))));
            ps.view();
        })
    });

    //选择圆形
    c.bench_function("select shape:circle", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::ChangeShape(
                Rectangle::default().into(),
            )));
            ps.view();
        })
    });

    //创建圆形
    c.bench_function("create circle", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 1.0 }),
            ))));
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Labor(Point { x: 10.0, y: 100.0 }),
            ))));
            ps.view();
        })
    });

    //粘贴曲线
    c.bench_function("paste curve", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::CurvePasted(Point {
                x: 10.0,
                y: 200.0,
            })));
            ps.view();
        })
    });

    //删除曲线
    c.bench_function("delete curve", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::RemoveCurve));
            ps.view();
        })
    });

    //输入color：r
    c.bench_function("input color :r", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::InputColorR(String::from("155")),
            )));
            ps.view();
        })
    });

    //输入color：g
    c.bench_function("input color :g", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::InputColorG(String::from("155")),
            )));
            ps.view();
        })
    });

    //输入color：b
    c.bench_function("input color :b", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::InputColorB(String::from("155")),
            )));
            ps.view();
        })
    });

    //输入color：a
    c.bench_function("input color :a", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::InputColorA(String::from("0.5")),
            )));
            ps.view();
        })
    });

    //滑动color：r
    c.bench_function("slide color :r", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::SlideColorR(0.55),
            )));
            ps.view();
        })
    });

    //滑动color：g
    c.bench_function("slide color :g", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::SlideColorG(0.55),
            )));
            ps.view();
        })
    });

    //滑动color：b
    c.bench_function("slide color :b", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::SlideColorB(0.55),
            )));
            ps.view();
        })
    });

    //滑动color：a
    c.bench_function("slide color :a", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::SlideColorA(0.55),
            )));
            ps.view();
        })
    });

    //输入width
    c.bench_function("input width", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::InputWidth(
                String::from("30"),
            ))));
            ps.view();
        })
    });

    //选择Linecap
    c.bench_function("select line-cap", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::LineCapSelected(EqLineCap::Butt),
            )));
            ps.view();
        })
    });

    //选择Linejoin
    c.bench_function("select line-join", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(
                CurveMessage::LineJoinSelected(EqLineJoin::Bevel),
            )));
            ps.view();
        })
    });

    //center curve
    c.bench_function("center curve", |b| {
        b.iter(|| {
            ps.update(Message::Edit(EditMessage::Curve(CurveMessage::Shape(
                ShapeMessage::Centered(Point { x: 30.0, y: 40.0 }),
            ))));
            ps.view();
        })
    });

    //清空曲线
    c.bench_function("clear curve", |b| {
        b.iter(|| ps.update(Message::Edit(EditMessage::Clear)));
        ps.view();
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
