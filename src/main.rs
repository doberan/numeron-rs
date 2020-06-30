// use termion::input::TermRead;
use termion::input::TermRead;
use std::io::Stdout;
use termion::raw::RawTerminal;
use std::io::Stdin;
use std::io::{stdout, stdin};
use termion::{event, raw::IntoRawMode};

use numeron_rs::game::{View, ViewManager, WriteFormat, Tern};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // template_viewを生成
    let view = vec![
        View {x: 5,     y: 1,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 42,    y: 1,  text: "NUMERON!!!!".to_string(),     format: WriteFormat::Text},
        View {x: 90,    y: 1,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 5,     y: 2,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 22,    y: 2,  text: "MY_ANSER".to_string(),        format: WriteFormat::Text},
        View {x: 49,    y: 2,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 50,    y: 2,  text: "".to_string(),                format: WriteFormat::TitleStart},
        View {x: 65,    y: 2,  text: "ENEMY_ANSER".to_string(),     format: WriteFormat::Text},
        View {x: 90,    y: 2,  text: "".to_string(),                format: WriteFormat::TitleEnd},
        View {x: 5,     y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 6,     y: 3,  text: "答えた数".to_string(),          format: WriteFormat::Text},
        View {x: 15,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 16,    y: 3,  text: "数字のみ一致".to_string(),       format: WriteFormat::Text},
        View {x: 25,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 26,    y: 3,  text: "数字も位置も一致".to_string(),    format: WriteFormat::Text},
        View {x: 49,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 50,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 51,    y: 3,  text: "答えた数".to_string(),         format: WriteFormat::Text},
        View {x: 60,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 61,    y: 3,  text: "数字のみ一致".to_string(),      format: WriteFormat::Text},
        View {x: 70,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 71,    y: 3,  text: "数字も位置も一致".to_string(),    format: WriteFormat::Text},
        View {x: 90,    y: 3,  text: "".to_string(),                format: WriteFormat::RecordPipe},
        View {x: 3,     y: 15, text: "input:".to_string(),          format: WriteFormat::Text}
    ];

    let mut view_manager = ViewManager::new(
        view,
        vec![0,1,2,3],
        0
    );
    view_manager.clear_view(&mut stdout);
    view_manager.template_view(&mut stdout);
    view_manager.show_tern(&mut stdout);
    view_manager.show_input_pointer(&mut stdout);
    view_manager.apply_view(&mut stdout);
    main_roop(&mut view_manager, stdin, &mut stdout);
}

  /// メインループ
fn main_roop(view_manager: &mut ViewManager, stdin: Stdin, stdout: &mut RawTerminal<Stdout>) {
    let mut input_x = 10_u16;
    view_manager.view(stdout, 10, 15, "", WriteFormat::Pointer);
    view_manager.view(stdout, 70, 15, &(format!("tern: {}", (view_manager.tern + 1).to_string())), WriteFormat::Pointer);
    for c in stdin.keys() {
        match c {
            Ok(event::Key::Char('k')) => {
                input_x = 10_u16;
                // 結果処理
                view_manager.clear_view(stdout);
                view_manager.show_result_view(stdout);
                view_manager.incliment_tern();

                view_manager.template_view(stdout);
                view_manager.show_tern(stdout);
                view_manager.show_input_pointer(stdout);

                view_manager.apply_view(stdout);
                view_manager.add_anser_list(Tern::My);
            },
            Ok(event::Key::Char('r')) => {
                input_x = 10_u16;
                view_manager.my_anser.remove(view_manager.tern as usize);

                view_manager.clear_view(stdout);
                view_manager.show_result_view(stdout);

                view_manager.template_view(stdout);
                view_manager.show_tern(stdout);
                view_manager.show_input_pointer(stdout);

                view_manager.apply_view(stdout);
                view_manager.add_anser_list(Tern::My);
            },
            Ok(event::Key::Char(a)) => {
                let index = input_x as i32 - 10;

                input_x = if input_x == 14_u16 {
                    14_u16
                } else {
                    view_manager.view(stdout, input_x, 15, &a.to_string(), WriteFormat::Text);
                    view_manager.apply_view(stdout);
                    view_manager.my_anser[view_manager.tern as usize][index as usize] = a as i32 - 48;
                    input_x + 1_u16
                };
            },
            Ok(event::Key::Ctrl('c')) => break,
            _ => {},
        }
    };
}