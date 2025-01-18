// 戦闘結果表示メニュー
mod battle_result_menu {
    use crate::battle_rules::Character;
    use crate::{battle_rules, Effect, Power, Probability, Skill, SpecialStatusType};
    use iced::widget::{pick_list, Column};
    use iced::Element;
    use rand::Rng;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub(crate) struct BattleResultMenu {
        skill: Skill,
        from: Rc<RefCell<Character>>,
        to: Option<Rc<RefCell<Character>>>,
        list_texts: Vec<String>,
        show_battle_end_button: bool,
        show_enemy_turn_button: bool,
        show_game_over_button: bool
    }

    #[derive(Debug, Clone)]
    pub(crate) enum Message {
        Initial,
        OnClickNext,
    }

    impl BattleResultMenu {

        pub(crate) fn new(skill: Skill, from: &mut Character, mut to: Option<Character>) -> Self {
            let mut menu = BattleResultMenu {
                skill,
                from: Rc::new(from.clone().into()),
                to: Some(Rc::new(to.unwrap().into())),
                list_texts: vec![],
                show_enemy_turn_button: false,
                show_battle_end_button: false,
                show_game_over_button: false
            };
            menu.effect_before_skill();
            match menu.skill.effect.clone() {
                Effect::Attack(probability, power) => {
                    // probability.percentage に基づき成功率を決める
                    let is_hit = menu.random_hit(probability.clone());
                    if is_hit {
                        let damage = menu.damage(power.clone());
                        let to_ref = menu.to.clone().unwrap();
                        menu.list_texts.push(format!(
                            "{} の {} が {} に {} のダメージを与えた！",
                            from.name,
                            menu.skill.clone().name,
                            to_ref.borrow_mut().name,
                            damage
                        ));
                        // target の残りHPを表示
                        menu.list_texts.push(format!(
                            "{} の HP: {}",
                            to_ref.borrow().name,
                            to_ref.borrow().hp
                        ));
                        if to_ref.borrow_mut().hp <= 0.0 {
                            menu.list_texts.push(format!("{} は たおれた！", to_ref.borrow_mut().name));
                        }
                    } else {
                        menu.list_texts.push(format!(
                            "{} の {} は失敗した...",
                            from.name,
                            menu.skill.clone().name,
                        ));
                    }
                }
                Effect::Heal(rate) => {
                    // おおよその回復値：最大 HP * rate
                    let heal_value = menu.from.borrow_mut().hp_max
                        * rate.percentage
                        * rand::thread_rng().gen_range((1.0)..=(1.2));
                    menu.heal(heal_value);
                    menu.list_texts
                        .push(format!("{} は {} の回復をした！", from.name, heal_value));
                }
                Effect::AddSpecialStatusToEnemy(probability, special_status) => {
                    if menu.random_hit(probability.clone()) {
                        menu.apply_special_status(special_status.clone());
                    }
                }
                Effect::AttackAndAddSpecialStatusToEnemy(
                    probability,
                    power,
                    probability_special_status,
                    special_status,
                ) => {
                    // probability.percentage に基づき成功率を決める
                    if menu.random_hit(probability.clone()) {
                        let damage = menu.damage(power.clone());
                        let to_ref = menu.to.clone().unwrap();
                        menu.list_texts.push(format!(
                            "{} の {} が {} に {} のダメージを与えた！",
                            from.name,
                            menu.skill.clone().name,
                            to_ref.borrow_mut().name,
                            damage
                        ));
                        if menu.random_hit(probability_special_status.clone()) {
                            menu.apply_special_status(special_status.clone());
                        }
                    }
                }
            }
            // target の HP が 0 以下ならば、戦闘終了。そうでなければ戦闘続行。
            if menu.to.as_ref().unwrap().borrow_mut().hp <= 0.0 {
                menu.list_texts.push(format!("{} は たおれた！", menu.to.as_ref().unwrap().borrow_mut().name));
                menu.show_battle_end_button = true;
            }
            menu.effect_after_skill();
            // from の HP が 0以下ならば、戦闘終了。
            if menu.from.borrow_mut().hp <= 0.0 {
                menu.list_texts.push(format!("{} は たおれた！", menu.from.borrow_mut().name));
                menu.show_game_over_button = true;
            }
            menu
        }

        pub(crate) fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {}
                Message::OnClickNext => {
                    // 何もしない
                }
            }
        }

        pub(crate) fn view(&self) -> Element<Message> {
            let mut column = Column::new();
            for text in &self.list_texts {
                column = column.push(iced::widget::text!("{}", text));
            }
            column = column.push(iced::widget::button("つぎへ").on_press(Message::OnClickNext));
            column.into()
        }

        fn random_hit(&mut self, probability: Probability) -> bool {
            rand::thread_rng().gen_range(0..100) < probability.percentage
        }

        fn damage(&mut self, power: Power) -> f32 {
            let random = rand::thread_rng().gen_range((1.0)..=(1.2));
            let power_value = power.value;
            let from = self.from.borrow_mut();
            let mut to = self.to.as_ref().unwrap().borrow_mut();
            let from_attack = from.attack;
            let to_defence = to.defence;
            let value = random * power_value * from_attack / to_defence;

            // ダメージの計算元の値を出力
            self.list_texts.push(format!("乱数: {}", random));
            // self.list_texts
            //     .push(format!("{} の攻撃力: {}", from.name, from_attack));
            // self.list_texts
            //     .push(format!("{} の防御力: {}", to.name, to_defence));
            // self.list_texts
            //     .push(format!("{} の威力: {}", self.skill.name, value));

            // ダメージを与える
            to.hp_decrease(value)
        }

        fn heal(&mut self, value: f32) {
            let mut from = self.from.borrow_mut();
            let tmp_hp = from.hp + value;
            if tmp_hp > from.hp_max {
                from.hp = from.hp_max;
            } else {
                from.hp = tmp_hp;
            }
        }

        fn effect_before_skill(&mut self) {
            let mut from = self.from.borrow_mut();
            if from.turn_of_poisoned > 0 {
                from.turn_of_poisoned -= 1
            };
            if from.turn_of_burned > 0 {
                from.turn_of_burned -= 1
            };
            if from.turn_of_falter > 0 {
                from.turn_of_falter -= 1
            };
            if from.turn_of_blackout > 0 {
                from.turn_of_blackout -= 1
            };
        }

        fn effect_after_skill(&mut self) {
            let mut from = self.from.borrow_mut();
            let hp_max = from.hp_max;
            if from.turn_of_poisoned > 0 {
                from.hp_decrease(hp_max / 16.0);
            }
            if from.turn_of_burned > 0 {
                from.hp_decrease(hp_max / 8.0);
            }
        }
        fn apply_special_status(&mut self, special_status: SpecialStatusType) {
            if let Some(to_ref) = self.to.as_ref() {
                let mut to = to_ref.borrow_mut();
                match special_status {
                    SpecialStatusType::Poisoned => {
                        if to.turn_of_poisoned < 1 {
                            self.list_texts
                                .push(format!("{} は 毒を受けた！", to.name.clone()));
                        } else {
                            self.list_texts
                                .push(format!("{} の 毒が長引く！", to.name.clone()));
                        }
                        to.turn_of_poisoned += 18;
                    }
                    SpecialStatusType::Burned => {
                        if to.turn_of_burned < 1 {
                            self.list_texts
                                .push(format!("{} は 火傷を受けた！", to.name.clone()));
                        } else {
                            self.list_texts
                                .push(format!("{} の 火傷が長引く！", to.name.clone()));
                        }
                        to.turn_of_burned += 8;
                    }
                    SpecialStatusType::Falter => {
                        if to.turn_of_falter < 1 && to.turn_of_frost < 0 {
                            self.list_texts
                                .push(format!("{} は ひるんだ！", to.name.clone()));
                            to.turn_of_falter = 2;
                        }
                    }
                    SpecialStatusType::BlackOut => {
                        if to.turn_of_blackout < 1 {
                            self.list_texts
                                .push(format!("{} は 目が見えない！", to.name.clone()));
                            to.turn_of_blackout = 2;
                        }
                    }
                    SpecialStatusType::Frozen => {
                        if { to.turn_of_falter < 1 && to.turn_of_frost < 0 } {
                            self.list_texts
                                .push(format!("{} は 凍りついた！", to.name.clone()));
                            to.turn_of_frost = 2;
                        }
                    }
                    SpecialStatusType::Feather => {
                        if { to.turn_of_feather < 1 } {
                            self.list_texts
                                .push(format!("{} は 羽を生やした！", to.name.clone()));
                        } else {
                            self.list_texts
                                .push(format!("{} の 羽が大きくなった！", to.name.clone()));
                        }
                        to.turn_of_feather += 5;
                    }
                }
            }
        }
    }
}

