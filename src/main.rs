// use termion::input::TermRead;
use termion::input::TermRead;
use std::io::Stdout;
use termion::raw::RawTerminal;
use std::io::Stdin;
use std::io::{stdout, stdin};
use termion::{event, raw::IntoRawMode};

use numeron_rs::game::{
    View,
    ViewManager,
    WriteFormat,
    Tern,
    InputView
};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // template_viewを生成
    let view = vec![
        View {x: 5,     y: 1,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 42,    y: 1,  text: "NUMERON!!!!".to_string(),     format: WriteFormat::Text},
        View {x: 90,    y: 1,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 5,     y: 2,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 38,    y: 2,  text: "anser: ".to_string(),         format: WriteFormat::Text},
        View {x: 90,    y: 2,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 5,     y: 3,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 22,    y: 3,  text: "MY_ANSER".to_string(),        format: WriteFormat::Text},
        View {x: 49,    y: 3,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 50,    y: 3,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 65,    y: 3,  text: "ENEMY_ANSER".to_string(),     format: WriteFormat::Text},
        View {x: 90,    y: 3,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 5,     y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 6,     y: 4,  text: "答えた数".to_string(),          format: WriteFormat::Text},
        View {x: 15,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 16,    y: 4,  text: "数字のみ一致".to_string(),       format: WriteFormat::Text},
        View {x: 25,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 26,    y: 4,  text: "数字も位置も一致".to_string(),    format: WriteFormat::Text},
        View {x: 49,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 50,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 51,    y: 4,  text: "答えた数".to_string(),         format: WriteFormat::Text},
        View {x: 60,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 61,    y: 4,  text: "数字のみ一致".to_string(),      format: WriteFormat::Text},
        View {x: 70,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 71,    y: 4,  text: "数字も位置も一致".to_string(),    format: WriteFormat::Text},
        View {x: 90,    y: 4,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 5,     y: 13, text: "Input number is [0 - 9].".to_string(),          format: WriteFormat::Text},
        View {x: 5,     y: 14, text: "[Ctrl + c : exit] [r : refresh input] [k : input number]".to_string(),          format: WriteFormat::Text},
        View {x: 5,     y: 15, text: "input:".to_string(),          format: WriteFormat::Text}
    ];

    let mut view_manager = ViewManager::new(
        view,
        vec![],
        0
    );

    view_manager.generate_anser();
    view_manager.clear_view(&mut stdout);
    view_manager.template_view(&mut stdout);
    view_manager.show_tern(&mut stdout);
    view_manager.show_input_pointer(&mut stdout);
    view_manager.apply_view(&mut stdout);
    main_roop(&mut view_manager, stdin, &mut stdout);
}

  /// メインループ
fn main_roop(view_manager: &mut ViewManager, stdin: Stdin, stdout: &mut RawTerminal<Stdout>) {
    let mut input_view = InputView::new();

    view_manager.show_tern(stdout);
    view_manager.show_anser(stdout);
    view_manager.show_input_pointer(stdout);

    for c in stdin.keys() {
        match c {
            Ok(event::Key::Char('k')) => {
                view_manager.apply_anser(&mut input_view, Tern::My);
                input_view.reset_input_x();
                // 結果処理
                view_manager.clear_view(stdout);
                view_manager.show_result_view(stdout);
                view_manager.incliment_tern();

                view_manager.template_view(stdout);
                view_manager.show_anser(stdout);
                view_manager.show_tern(stdout);
                view_manager.show_input_pointer(stdout);

                view_manager.apply_view(stdout);
            },
            Ok(event::Key::Char('r')) => {
                input_view.reset_input_x();
                view_manager.clear_view(stdout);
                view_manager.show_result_view(stdout);

                view_manager.template_view(stdout);
                view_manager.show_anser(stdout);
                view_manager.show_tern(stdout);
                view_manager.show_input_pointer(stdout);

                view_manager.apply_view(stdout);
            },
            Ok(event::Key::Char(a)) => {
                if a as i32 - 48 >= 0 && a as i32 - 48 <= 9 {
                    input_view.set_input_value(a);
                    input_view.show_input_from_viewmanager(stdout, view_manager);
                    input_view.next_input_x();
                    view_manager.apply_view(stdout);
                }
           },
            Ok(event::Key::Ctrl('c')) => return,
            _ => {},
        }
    };
}