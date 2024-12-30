use crate::RandomCollection::{RandomEnemyCollection, RandomItemCollection};
use iced::widget::{pick_list, Column, Text};
use iced::{Element, Font};
use rand::seq::{SliceRandom};
use serde::Deserialize;

pub fn main() -> iced::Result {
    iced::application("Rustern-battle", App::update, App::view)
        .default_font(Font::with_name("ヒラギノ角ゴシック"))
        .run()
}

mod use_skill_menu {
    use crate::{Rarity, Skill, Skills};
    use iced::widget::{pick_list, Column};
    use iced::Element;

    pub(crate) struct UseSkillMenu {
        pub visible: bool,
        skills: Skills,
        skill: Option<Skill>,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Initial,
        OnSelectSkill(Skill),
        OnClickNext,
        OnClickBack,
    }

    impl UseSkillMenu {
        pub fn new(skills: Skills, visible: bool) -> Self {
            UseSkillMenu {
                visible,
                skills,
                skill: None,
            }
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {
                    // 画面を表示する
                    self.visible = true;
                    // 何も選択していない状態にする
                    self.skill = None;
                }
                Message::OnSelectSkill(skill) => {
                    // 選択しているスキルを更新する
                    self.skill = Some(skill)
                }
                Message::OnClickNext => {
                    // 画面を非表示にする
                    self.visible = false;
                }
                Message::OnClickBack => {
                    // 選択を解除する
                    self.skill = None;
                    // 画面を非表示にする
                    self.visible = false;
                }
            }
        }

        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();
            if !self.visible {
                return column.into();
            }
            column = column.push("どの　スキルを　つかう？");
            column = column.push(
                pick_list(
                    self.skills.random_pick(Rarity { value: 1 }, 2),
                    self.skill.clone(),
                    Message::OnSelectSkill
                )

            );

            match &self.skill {
                Some(_skill) => {
                    // 操作が選択されている場合、次へ進むためのボタンを表示する
                    let confirm =
                        iced::widget::button(
                            "この　スキルで　よい"
                        ).on_press(Message::OnClickNext);
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

// 戦闘操作メニュー。
// 「スキルをつかう」または「アイテムをつかう」を選択可能。
mod battle_operation_menu {
    use iced::widget::{pick_list, Column};
    use iced::Element;

    #[derive(Debug, Clone)]
    pub enum Message {
        Initial,
        OnSelectOperation(Operation),
        OnClickNext,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Operation {
        ShowSkills, // スキルをつかう
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

    pub struct BattleOperationMenu {

        // この画面を表示するかどうか
        pub visible: bool,

        // 選択している操作
        pub operation: Option<Operation>,
    }

    impl BattleOperationMenu {
        pub fn new() -> Self {
            BattleOperationMenu { visible: false, operation: None }
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Initial => {
                    // 画面を表示する
                    self.visible = true;
                    // 何も選択していない状態にする
                    self.operation = None;
                }
                Message::OnSelectOperation(operation) => {
                    // 選択している操作を更新する
                    self.operation = Some(operation);
                }
                Message::OnClickNext => {
                    // 画面を非表示にする
                    self.visible = false;
                }
            }
        }

        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();

            if !self.visible {
                // 何も表示しない
                return column.into();
            }
            column = column.push("どうする？");

            // 操作を選択するドロップダウンリストの作成
            let operations =
                vec![
                    Operation::ShowSkills,
                    Operation::ShowItemContainers,
                ];
            column = column.push(pick_list(
                operations,
                self.operation,
                Message::OnSelectOperation
            ));

            match self.operation {
                Some(_operation) => {
                    // 操作が選択されている場合、次へ進むためのボタンを表示する
                    let confirm =
                        iced::widget::button(
                            "この　こうどうで　よい"
                        ).on_press(Message::OnClickNext);
                    column = column.push(confirm);
                }
                None => {}
            }

            column.into()
        }
    }
}

struct App {
    //サブビュー
    battle_operation_menu: battle_operation_menu::BattleOperationMenu,
    use_skill_menu: use_skill_menu::UseSkillMenu,
    //データ
    scenario: Vec<Message>,
    scenario_idx: usize,
    master_data: MasterData,
    system_info: String,
    items_for_get: Vec<Item>,
    owned_items: Vec<ItemContainer>,
    selected_item: Option<Item>,
    encountered_enemies: Vec<Enemy>,
    //表示制御
    show_next_button: bool,
    show_items_for_pick: bool,
}


#[derive(Debug, Clone, Deserialize)]
struct Items {
    items: Vec<Item>,
}

impl Items {

    //! 指定したレアリティのアイテムを、指定した数だけ持つ配列を返す
    fn random_pick(&self, rarity: Rarity, count: usize) -> Vec<Item> {
        let filtered: Vec<Item> = self.items
            .iter()
            .filter(|item| item.rarity == rarity)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ItemContainer {
    item: Item,
    amount: usize,
}

impl std::fmt::Display for ItemContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (残り: {}個)", self.item.name.clone(), self.amount)
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Item {
    name: String,
    rarity: Rarity,
    effect: Effect,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Rarity {
    value: u8,
}

impl Rarity {
    fn new(value: u8) -> Rarity {
        Rarity { value }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Level {
    value: u8,
}

impl Level {
    fn new(value: u8) -> Level {
        Level { value }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct Skills {
    skills: Vec<Skill>,
}

impl Skills {

    //! 指定したレベルのスキルを、指定した数だけ持つ配列を返す
    fn random_pick(&self, rarity: Rarity, count: usize) -> Vec<Skill> {
        let filtered: Vec<Skill> = self.skills
            .iter()
            .filter(|skill| skill.rarity == rarity)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Skill {
    name: String,
    rarity: Rarity,
    effect: Effect,
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum Effect {
    Attack( //攻撃
            Probability, //成功率
            Power, //威力
    ),
    Heal( //回復
          Ratio //最大HPに対する回復割合
    ),
    AddSpecialStatusToEnemy( //敵に特殊状態を付与
                             Probability, //成功率
                             SpecialStatus, //特殊状態
    ),
    AttackAndAddSpecialStatusToEnemy( //敵に攻撃しつつ特殊状態を付与
                                      Probability, //成功率
                                      Power, //威力
                                      Probability, //特殊状態付与確率
                                      SpecialStatus, //特殊状態
    ),
}

#[allow(dead_code)]
fn use_skill(skill_type: Effect) {
    match skill_type {
        Effect::Attack(_, _) => {}
        Effect::Heal(_) => {}
        Effect::AddSpecialStatusToEnemy(_, _) => {}
        Effect::AttackAndAddSpecialStatusToEnemy(_, _, _, _) => {}
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Power {
    value: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Probability {
    percentage: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Ratio {
    percentage: f32,
}

#[allow(dead_code)]
enum AdditionalEffect {
    AddSpecialStatus(SpecialStatus, Probability),
    DrainHP(Ratio),
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
enum SpecialStatus {
    Poisoned, // 継続ダメージ(最大HPの16分の1)、攻撃力ダウン
    Burned, // 継続ダメージ(最大HPの8分の1)
    Falter, // 回避不能、行動不能、防御力ダウン
    BlackOut, // 回避不能、攻撃が外れる、追加効果無効
    Frozen,  // 回避不能、追加効果無効
    Feather, // 回避率上昇、防御力ダウン
}

#[derive(Debug, Clone, Deserialize)]
struct MasterData {
    enemies: Enemies,
    items: Items,
    skills: Skills,
}

#[derive(Debug, Clone, Deserialize)]
struct Enemies {
    enemies: Vec<Enemy>,
}

impl Enemies {
    //! 指定したレベルの敵を、指定した数だけ持つ配列を返す
    fn random_pick(&self, level: Level, count: usize) -> Vec<Enemy> {
        let filtered: Vec<Enemy> = self.enemies
            .iter()
            .filter(|enemy| enemy.level == level)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Enemy {
    name: String,
    level: Level,
    hp: f32,
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Info(String),
    UpdateSelectorAndInfo(RandomCollection, String),
    ShowItemsForPick,
    HideItemsForPick,
    WaitingSelectItemByUser(Item),
    GiveSelectedItemForUser,
    BattleOperationMenu(battle_operation_menu::Message),
    UseSkillMenu(use_skill_menu::Message),
}

#[derive(Clone, Debug, PartialEq)]
enum BattleOperation {
    ShowSkills,
    ShowItemStocks,
}

impl std::fmt::Display for BattleOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BattleOperation::ShowSkills => {
                write!(f, "スキルをつかう")
            }
            BattleOperation::ShowItemStocks => {
                write!(f, "アイテムをつかう")
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Clone)]
enum PageOfBattleOperation {
    Root,
    Skills,
    ItemContainers,
    Enemies,
}

impl App {
    // ゲーム開始時の処理
    fn new() -> Self {
        // マスタデータ(YAML)読み込み
        let dir = std::env::var("RUSTERN_DIR").unwrap();
        let file_name = "example.yml";
        let file_path = std::path::PathBuf::from(format!("{}/{}", dir, file_name));
        let yaml_contents = std::fs::read_to_string(&file_path).unwrap();
        let master_data: MasterData = serde_yaml::from_str(&yaml_contents).unwrap();
        let usable_skills = master_data.skills.random_pick(Rarity { value: 1 }, 2);

        // 初期化
        let first_message: String = "おうさま：おお　ゆうしゃよ　まおうを　たおしに　ゆくのじゃ".into();
        let skills_for_menu: Skills = Skills { skills: usable_skills };
        Self {
            battle_operation_menu: battle_operation_menu::BattleOperationMenu::new(),
            use_skill_menu: use_skill_menu::UseSkillMenu::new(skills_for_menu, false),
            //データ
            scenario: vec![
                Message::Info(first_message.clone()),
                Message::Info("おうさま：アイテムを　ひとつ　さずけよう。".into()),
                Message::UpdateSelectorAndInfo(RandomItemCollection(Rarity::new(1), 2), "どの　アイテムを　もらう？".into()),
                Message::ShowItemsForPick,
                Message::GiveSelectedItemForUser,
                Message::Info("さあ　まおうを　たおす　たびの　はじまりだ。".into()),
                Message::Info("てきが　あらわれた！".into()),
                Message::UpdateSelectorAndInfo(RandomEnemyCollection(Level::new(1), 1), "".into()),
                Message::BattleOperationMenu(battle_operation_menu::Message::Initial),
            ],
            scenario_idx: 0,
            master_data,
            system_info: first_message.clone(),
            items_for_get: vec![],
            owned_items: vec![],
            selected_item: None,
            encountered_enemies: vec![],
            //表示制御
            show_next_button: true,
            show_items_for_pick: false,
        }
    }

    fn hide_all_components(&mut self) {
        self.show_next_button = false;
        self.show_items_for_pick = false;
    }
    fn update(&mut self, message: Message) {
        self.hide_all_components();
        match message {
            Message::Next => {
                // シナリオを進める
                self.scenario_idx += 1;
                if let Some(msg) = self.scenario.get(self.scenario_idx) {
                    self.update(msg.clone());
                }
            }
            Message::ShowItemsForPick => {
                // アイテムを貰うためのドロップダウンリストを表示する
                self.show_items_for_pick = true;
            }
            Message::HideItemsForPick => {
                // アイテムを貰うためのドロップダウンリストを隠す
                self.show_items_for_pick = false;
            }
            Message::Info(info) => {
                // 画面上部に情報を表示したうえで、次に進むボタンを表示する
                self.system_info = info;
                self.show_next_button = true;
            }
            Message::UpdateSelectorAndInfo(random_collection, info) => {
                // ドロップダウンリストの内容を変更しつつ、画面上部に情報を表示する
                match random_collection {
                    RandomItemCollection(rarity, count) => {
                        self.items_for_get = self.master_data.items.random_pick(rarity, count as usize);
                    }
                    RandomEnemyCollection(level, count) => {
                        self.encountered_enemies = self.master_data.enemies.random_pick(level, count as usize);
                    }
                };
                self.system_info = info;
                self.update(Message::Next);
            }
            Message::WaitingSelectItemByUser(item) => {
                // ユーザーがドロップダウンリストから選ぶのを待つ
                self.selected_item = Some(item);
                self.show_items_for_pick = true;
                if self.selected_item.is_some() {
                    self.show_next_button = true;
                }
            }
            Message::GiveSelectedItemForUser => {
                // 選択されたアイテムを得る
                if let Some(selected_item) = self.selected_item.clone() {
                    if let Some(existing_item) = self.owned_items.iter_mut().find(|container| container.item == selected_item) {
                        existing_item.amount += 1;
                    } else {
                        self.owned_items.push(ItemContainer {
                            item: selected_item.clone(),
                            amount: 1,
                        });
                    }
                    self.system_info = format!("{}　を　てにいれた！", selected_item.name);
                    self.selected_item = None;
                    self.items_for_get = vec![];
                } else {
                    self.system_info = "アイテムが　えらばれて　いない。".to_string();
                }
                self.show_next_button = true;
            }
            Message::BattleOperationMenu(message) => {
                // 戦闘操作メニューを表示する
                match message {
                    battle_operation_menu::Message::Initial => {
                    }
                    battle_operation_menu::Message::OnSelectOperation(_operation) => {
                    }
                    battle_operation_menu::Message::OnClickNext => {
                        match self.battle_operation_menu.operation {
                            None => {}
                            Some(operation) => {
                                match operation {
                                    battle_operation_menu::Operation::ShowSkills => {
                                        self.use_skill_menu.visible = true;
                                    }
                                    battle_operation_menu::Operation::ShowItemContainers => {
                                    }
                                }

                            }
                        }
                    }
                }
                self.battle_operation_menu.visible = true;
                self.battle_operation_menu.update(message);
            }
            Message::UseSkillMenu(message) => {
                match message {
                    use_skill_menu::Message::Initial => {
                        self.use_skill_menu.visible = true;
                    }
                    use_skill_menu::Message::OnSelectSkill(_) => {
                        self.use_skill_menu.visible = true;
                    }
                    use_skill_menu::Message::OnClickNext => {
                        self.use_skill_menu.visible = false;
                    }
                    use_skill_menu::Message::OnClickBack => {
                        self.use_skill_menu.visible = false;
                        self.battle_operation_menu.visible = true;
                    }
                }
                self.use_skill_menu.update(message);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new();
        let system_info = Text::new(self.system_info.as_str());
        column = column.push(system_info);

        // サブビューの表示
        column = column.push(self.battle_operation_menu.view().map(|message|Message::BattleOperationMenu(message)));
        column = column.push(self.use_skill_menu.view().map(|message|Message::UseSkillMenu(message)));

        // ゲームの初回でおうさまからアイテムを貰う処理
        if self.show_items_for_pick {
            // アイテムリスト
            let item_candidates = pick_list(
                self.items_for_get.clone(),
                self.selected_item.clone(),
                Message::WaitingSelectItemByUser,
            );
            column = column.push(item_candidates);
        }

        if self.show_next_button {
            column = column.push(iced::widget::button("つぎへ").on_press(Message::Next));
        }
        column.into()
    }
}

#[derive(Debug, Clone)]
enum RandomCollection {
    RandomItemCollection(Rarity, i8),
    RandomEnemyCollection(Level, i8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_selected_item() {
        // テスト用の初期データを作成
        let mut app = App::new();
        let test_item = Item {
            name: "ポーション".to_string(),
            rarity: Rarity::new(1),
            effect: Effect::Heal(Ratio { percentage: 0.1 }),
        };
        app.selected_item = Some(test_item.clone());
        app.owned_items = vec![];

        // 処理を実行
        app.update(Message::GiveSelectedItemForUser);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 1);
        assert_eq!(app.owned_items[0].item, test_item);
        assert_eq!(app.owned_items[0].amount, 1);
        assert_eq!(app.system_info, format!("{}　を　てにいれた！", test_item.name));

        // アイテムを追加して再度テスト
        app.selected_item = Some(test_item.clone());
        app.update(Message::GiveSelectedItemForUser);
        assert_eq!(app.owned_items.len(), 1); // 所持アイテム数は変わらない
        assert_eq!(app.owned_items[0].amount, 2); // 同じアイテムの数が増える
    }

    #[test]
    fn test_get_selected_item_no_selection() {
        // テスト用の初期データを作成
        let mut app = App::new();
        app.selected_item = None;
        app.owned_items = vec![];

        // 処理を実行
        app.update(Message::GiveSelectedItemForUser);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 0);
        assert_eq!(app.system_info, "アイテムが　えらばれて　いない。");
    }
}
