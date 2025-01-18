mod target_enemy_menu {
    use crate::{Character, Characters, Level, Rarity};
    use iced::widget::{pick_list, Column};
    use iced::Element;

    #[derive(Debug, Clone)]
    pub enum Message {
        Initial,
        OnSelectEnemy(Character),
        OnClickNext,
        OnClickBack,
    }

    pub struct TargetEnemyMenu {
        enemies: Characters,
        pub(crate) enemy: Option<Character>,
    }

    impl TargetEnemyMenu {
        pub(crate) fn new(enemies: Characters) -> Self {
            TargetEnemyMenu {
                enemies,
                enemy: None,
            }
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {
                    // 何も選択していない状態にする
                    self.enemy = None;
                }
                Message::OnSelectEnemy(enemy) => {
                    // 選択している敵を更新する
                    self.enemy = Some(enemy)
                }
                Message::OnClickNext => {}
                Message::OnClickBack => {
                    // 選択を解除する
                    self.enemy = None;
                }
            }
        }

        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();
            column = column.push("どの　てきを　ねらう？");
            column = column.push(pick_list(
                self.enemies.characters.clone(),
                self.enemy.clone(),
                Message::OnSelectEnemy,
            ));

            match &self.enemy {
                Some(_skill) => {
                    // 敵が選択されている場合、次へ進むためのボタンを表示する
                    let confirm =
                        iced::widget::button("この　てきで　よい").on_press(Message::OnClickNext);
                    column = column.push(confirm);
                }
                None => {}
            }

            // 戻るボタン
            column = column.push(iced::widget::button("もどる").on_press(Message::OnClickBack));
            column.into()
        }
    }
}

pub use target_enemy_menu::TargetEnemyMenu;
pub use target_enemy_menu::Message;
