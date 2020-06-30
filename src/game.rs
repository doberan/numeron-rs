//! ゲーム管理
use termion::*;
use std::io::{Write, Stdout};
use rand::prelude::*;
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
      enemy_anser: vec![],
      my_anser: vec![],
      tern,
    }
  }

  pub fn generate_anser(&mut self) {
    let mut rng = rand::thread_rng();
    let rand_vec: Vec<i32> = (0..4).map(|_| rng.gen_range(0, 9)).collect();
    self.anser = rand_vec;
  }

  pub fn show_anser(&mut self, stdout: &mut RawTerminal<Stdout>) {
    let result_y = 2_u16;
    let mut result_x = 48_u16;
    self.view(stdout, result_x,  result_y,  "", WriteFormat::TitleStart);
    result_x += 1;
    self.view(stdout, result_x,  result_y, " ", WriteFormat::Text);
    result_x += 1;
    for result in self.anser.clone().iter_mut() {
      self.view(stdout, result_x,  result_y, &result.to_string(), WriteFormat::Text);
      result_x += 1;
      self.view(stdout, result_x,  result_y, " ", WriteFormat::Text);
      result_x += 1;
    }
    self.view(stdout, result_x,  result_y,  "", WriteFormat::TitleEnd);
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
    let mut result_y: u16 = 5;
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
  pub fn apply_anser(&mut self, input_view: &mut InputView, tern: Tern) {
    let _ = match tern {
      Tern::My => {
        self.my_anser.push(vec![]);
        self.my_anser[self.tern as usize] = input_view.input_value.clone();
      },
      Tern::Enemy => {
        self.enemy_anser.push(vec![]);
        self.enemy_anser[self.tern as usize] = input_view.input_value.clone();
      }
    };
  }
}

/// 入力用
#[derive(Clone)]
pub struct InputView {
  input_x: u16,
  input_y: u16,
  pub input_value: Vec<i32>,
}

impl InputView {

  pub const INPUT_MAX: u16 = 15;

  pub fn new() -> Self {
    Self {
      input_x: 11_u16,
      input_y: 15_u16,
      input_value: vec![],
    }
  }

  /// inputのｘ軸をリセットする
  pub fn reset_input_x(&mut self) {
    self.input_x = 11_u16;
    self.input_value = vec![];
  }

  /// inputのｘ軸を次にずらす
  pub fn next_input_x(&mut self) {
    if self.input_x < InputView::INPUT_MAX { self.input_x += 1_u16; };
  }


  pub fn set_input_value(&mut self, value: char) {
    if self.input_x < InputView::INPUT_MAX {
      self.input_value.push(value as i32 - 48);
    }
  }

  /// inputを描画する
  pub fn show_input_from_viewmanager(&mut self, stdout: &mut RawTerminal<Stdout>, view_manager: &mut ViewManager) {
    if self.input_x < InputView::INPUT_MAX {
      view_manager.view(
        stdout,
        self.input_x,
        self.input_y,
        &(self.input_value[(self.input_x - 11) as usize].to_string()),
        WriteFormat::Text
      );
    }
  }
}