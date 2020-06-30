//! ゲーム管理
use termion::*;
use std::io::{Write, Stdout};
use termion::{
  raw::RawTerminal
};


#[derive(Clone)]
/// ターン
pub enum Tern {
  /// 自分
  My,
  /// 敵
  Enemy
}

#[derive(Clone)]
pub enum WriteFormat {
  /// テキスト
  Text,
  /// タイトル開始
  TitleStart,
  /// タイトル終了
  TitleEnd,
  /// テーブルのパイプ
  RecordPipe,
  /// ポインタ
  Pointer,
}

/// テキスト描画用オブジェクト
#[derive(Clone)]
pub struct View {
  /// x座標
  pub x: u16,
  /// y座標
  pub y: u16,
  /// テキスト
  pub text: String,
  /// フォーマット
  pub format: WriteFormat
}

pub struct ViewManager {
  /// 描画用
  pub view: Vec<View>,
  /// 答え
  pub anser: Vec<i32>,
  /// 敵の答え
  pub enemy_anser: Vec<Vec<i32>>,
  /// 自分の答え
  pub my_anser: Vec<Vec<i32>>,
  /// ターン
  pub tern: i32
}

impl ViewManager {
  pub fn new(view: Vec<View>, anser: Vec<i32>, tern: i32) -> Self {
    Self {
      view,
      anser,
      enemy_anser: vec![vec![0,0,0,0]],
      my_anser: vec![vec![0,0,0,0]],
      tern,
    }
  }

  /// templateを描画する
  pub fn template_view(&mut self, mut stdout: &mut RawTerminal<Stdout>) {
    // タイトル位置
    for v in self.view.clone() {
      &self.view(
        &mut stdout,
        v.x,
        v.y,
        v.text.as_str(),
        v.format);
    };
  }

  /// 結果を描画する
  pub fn show_result_view(&mut self, stdout: &mut RawTerminal<Stdout>) {
    let mut result_y: u16 = 4;
    for result in self.my_anser.clone().iter_mut() {
      let mut result_x: u16 = 5;
      self.view(stdout, result_x,  result_y,  "", WriteFormat::RecordPipe);
      for value in result.iter_mut() {
        result_x += 1;
        self.view(stdout, result_x,  result_y,  &value.to_string(), WriteFormat::Text);
      }
      self.view(stdout, 15, result_y,  "", WriteFormat::RecordPipe);
      result_y += 1;
    }
  }

  /// 描画をセットする
  pub fn view(&mut self, stdout: &mut RawTerminal<Stdout>, x: u16, y: u16, view_str: &str, format: WriteFormat) {
    match format {
      WriteFormat::Text         => { write!(stdout, "{}{}{}", color::Fg(color::Rgb(255, 255, 255)), termion::cursor::Goto(x, y), view_str).unwrap(); },
      WriteFormat::TitleStart   => { write!(stdout, "{}{}[", color::Fg(color::Rgb(0, 255, 0)), termion::cursor::Goto(x, y)).unwrap(); },
      WriteFormat::TitleEnd     => { write!(stdout, "{}{}]", color::Fg(color::Rgb(0, 255, 0)), termion::cursor::Goto(x, y)).unwrap(); },
      WriteFormat::RecordPipe   => { write!(stdout, "{}{}|", color::Fg(color::Rgb(0, 128, 128)), termion::cursor::Goto(x, y)).unwrap(); },
      WriteFormat::Pointer      => { write!(stdout, "{}{}", termion::cursor::Goto(x, y), termion::cursor::Show).unwrap(); },
    };
  }

  /// viewを初期化する
  pub fn clear_view(&mut self, stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::clear::All).unwrap();
  }

  /// ターンを表示する
  pub fn show_tern(&mut self, stdout: &mut RawTerminal<Stdout>) {
    self.view(stdout, 75, 15, &(format!("tern: {}", (self.tern + 1).to_string())), WriteFormat::Text);
  }

  /// 入力待ち用のポインタ位置出力
  pub fn show_input_pointer(&mut self, stdout: &mut RawTerminal<Stdout>) {
    self.view(stdout, 11, 15, "", WriteFormat::Pointer);
  }

  /// viewを反映させる
  pub fn apply_view(&mut self, stdout: &mut RawTerminal<Stdout>) {
    stdout.flush().unwrap();
  }

  /// ターンを経過させる
  pub fn incliment_tern(&mut self) {
    self.tern += 1;
  }

  /// self.tern分の結果格納要素を追加する
  pub fn add_anser_list(&mut self, tern: Tern) {
    let _ = match tern {
      Tern::My => {
        self.my_anser.push(vec![]);
        self.my_anser[self.tern as usize] = vec![0,0,0,0];
      },
      Tern::Enemy => {
        self.enemy_anser.push(vec![]);
        self.enemy_anser[self.tern as usize] = vec![0,0,0,0];
      }
    };
  }
}
