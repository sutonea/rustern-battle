mod use_skill_menu {
    use crate::{Rarity, Skill, Skills};
    use iced::widget::{pick_list, Column};
    use iced::Element;

    #[derive(Debug, Clone)]
    pub struct UseSkillMenu {
        skills: Skills,
        pub(crate) skill: Option<Skill>,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Initial,
        OnSelectSkill(Skill),
        OnClickNext,
        OnClickBack,
    }

    impl UseSkillMenu {
        pub fn new(skills: Skills) -> Self {
            UseSkillMenu {
                skills,
                skill: None,
            }
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {
                    // 何も選択していない状態にする
                    self.skill = None;
                }
                Message::OnSelectSkill(skill) => {
                    // 選択しているスキルを更新する
                    self.skill = Some(skill)
                }
                Message::OnClickNext => {
                    // 何もしない
                }
                Message::OnClickBack => {
                    // 選択を解除する
                    self.skill = None;
                }
            }
        }

        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();
            column = column.push("どの　スキルを　つかう？");
            column = column.push(pick_list(
                self.skills.skills.clone(),
                self.skill.clone(),
                Message::OnSelectSkill,
            ));

            match &self.skill {
                Some(_skill) => {
                    // 操作が選択されている場合、次へ進むためのボタンを表示する
                    let confirm =
                        iced::widget::button("この　スキルで　よい").on_press(Message::OnClickNext);
                    column = column.push(confirm);
                }
                None => {
                    // 操作が選択されている場合、次へ進むためのボタンを非活性で表示する
                    let confirm = iced::widget::button("この　スキルで　よい");
                    column = column.push(confirm);
                }
            }

            // 戻るボタン
            column = column.push(iced::widget::button("もどる").on_press(Message::OnClickBack));
            column.into()
        }
    }
}
pub use use_skill_menu::UseSkillMenu;
pub use use_skill_menu::Message;
