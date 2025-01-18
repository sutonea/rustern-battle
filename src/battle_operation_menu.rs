// 戦闘操作メニュー。
// 「スキルをつかう」または「アイテムをつかう」を選択可能。
mod battle_operation_menu {
    use iced::widget::{pick_list, Column};
    use iced::Element;

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Initial,
        OnSelectOperation(Operation),
        OnClickNext,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Operation {
        ShowSkills,         // スキルをつかう
        ShowItemContainers, // アイテムをつかう
    }

    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Operation::ShowSkills => {
                    write!(f, "スキルをつかう")
                }
                Operation::ShowItemContainers => {
                    write!(f, "アイテムをつかう")
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BattleOperationMenu {
        // 選択している操作
        pub operation: Option<Operation>,
    }

    impl BattleOperationMenu {
        pub fn new() -> Self {
            BattleOperationMenu { operation: None }
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {
                    // 何も選択していない状態にする
                    self.operation = None;
                }
                Message::OnSelectOperation(operation) => {
                    // 選択している操作を更新する
                    self.operation = Some(operation);
                }
                Message::OnClickNext => {
                    // 何もしない
                }
            }
        }

        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();

            column = column.push("どうする？");

            // 操作を選択するドロップダウンリストの作成
            let operations = vec![Operation::ShowSkills, Operation::ShowItemContainers];
            column = column.push(pick_list(
                operations,
                self.operation,
                Message::OnSelectOperation,
            ));

            match self.operation {
                Some(_operation) => {
                    // 操作が選択されている場合、次へ進むためのボタンを表示する
                    let confirm = iced::widget::button("この　こうどうで　よい")
                        .on_press(Message::OnClickNext);
                    column = column.push(confirm);
                }
                None => {
                    // 操作が選択されていない場合、非活性のボタンを表示する
                    let confirm = iced::widget::button("この　こうどうで　よい");
                    column = column.push(confirm);
                }
            }

            column.into()
        }
    }
}

pub use battle_operation_menu::BattleOperationMenu;
pub use battle_operation_menu::Message;