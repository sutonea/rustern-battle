use crate::RandomCollection::{RandomEnemyCollection, RandomItemCollection};
use iced::widget::{pick_list, Column, Text};
use iced::{Element, Font};
use iced::application::Update;
use rand::seq::{SliceRandom};
use serde::Deserialize;

pub fn main() -> iced::Result {
    iced::application("Rustern-battle", App::update, App::view)
        .default_font(Font::with_name("ヒラギノ角ゴシック"))
        .run()
}

mod calc_battle_result {
    use crate::{Enemy, Player};

    trait CalcBattleResult {
        fn player_turn(player: Player, enemy: Enemy) {

        }
    }
}

mod use_skill_menu {
    use crate::{Rarity, Skill, Skills};
    use iced::widget::{pick_list, Column};
    use iced::Element;

    #[derive(Debug, Clone)]
    pub(crate) struct UseSkillMenu {
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
            column = column.push(
                pick_list(
                    self.skills.skills.clone(),
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
                None => {
                    // 操作が選択されている場合、次へ進むためのボタンを非活性で表示する
                    let confirm =
                        iced::widget::button(
                            "この　スキルで　よい"
                        );
                    column = column.push(confirm);
                }
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

    #[derive(Debug, Clone, Copy)]
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
                None => {
                    // 操作が選択されていない場合、非活性のボタンを表示する
                    let confirm =
                        iced::widget::button(
                            "この　こうどうで　よい"
                        );
                    column = column.push(confirm);

                }
            }

            column.into()
        }
    }
}

mod target_enemy_menu {
    use iced::Element;
    use iced::widget::{pick_list, Column};
    use crate::{Enemies, Enemy, Level, Rarity};

    #[derive(Debug, Clone)]
    pub(crate) enum Message {
        Initial,
        OnSelectEnemy(Enemy),
        OnClickNext,
        OnClickBack,
    }

    pub(crate) struct TargetEnemyMenu {
        enemies: Enemies,
        pub(crate) enemy: Option<Enemy>,
    }

    impl TargetEnemyMenu {
        pub(crate) fn new(enemies: Enemies) -> Self {
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
                    // 選択しているスキルを更新する
                    self.enemy = Some(enemy)
                }
                Message::OnClickNext => {
                }
                Message::OnClickBack => {
                    // 選択を解除する
                    self.enemy = None;
                }
            }
        }


        pub fn view(&self) -> Element<Message> {
            let mut column = Column::new();
            column = column.push("どの　てきを　ねらう？");
            column = column.push(
                pick_list(
                    self.enemies.enemies.clone(),
                    self.enemy.clone(),
                    Message::OnSelectEnemy
                )

            );

            match &self.enemy {
                Some(_skill) => {
                    // 敵が選択されている場合、次へ進むためのボタンを表示する
                    let confirm =
                        iced::widget::button(
                            "この　てきで　よい"
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

struct App {
    //サブビュー
    battle_operation_menu: Option<battle_operation_menu::BattleOperationMenu>,
    use_skill_menu: Option<use_skill_menu::UseSkillMenu>,
    target_enemy_menu: Option<target_enemy_menu::TargetEnemyMenu>,
    //データ
    scenario: Vec<Message>,
    scenario_idx: usize,
    master_data: MasterData,
    system_info: String,
    encountered_enemies_info: Vec<String>,
    items_for_get: Vec<Item>,
    owned_items: Vec<ItemContainer>,
    selected_item: Option<Item>,
    usable_skills: Skills,
    encountered_enemies: Option<Enemies>,
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

struct ItemContainers {
    containers: Vec<ItemContainer>
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

// スキルの日本語表現。スキル名とする
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

// 力を表す値オブジェクト
#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Power {
    value: f32,
}

// 発生確率を表す値オブジェクト
#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Probability {
    percentage: f32,
}

// 割合を表す値オブジェクト
#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Ratio {
    percentage: f32,
}

// スキルの追加効果
#[allow(dead_code)]
enum AdditionalEffect {
    AddSpecialStatus(SpecialStatus, Probability),
    DrainHP(Ratio),
}


// 特殊状態の列挙型
#[derive(Debug, Clone, Deserialize, PartialEq)]
enum SpecialStatus {
    Poisoned, // 継続ダメージ(最大HPの16分の1)、攻撃力ダウン
    Burned, // 継続ダメージ(最大HPの8分の1)
    Falter, // 回避不能、行動不能、防御力ダウン
    BlackOut, // 回避不能、攻撃が外れる、追加効果無効
    Frozen,  // 回避不能、追加効果無効
    Feather, // 回避率上昇、防御力ダウン
}

// yaml から読み込む想定のデータ
#[derive(Debug, Clone, Deserialize)]
struct MasterData {
    enemies: Enemies,
    items: Items,
    skills: Skills,
}

// 敵の集まり。
#[derive(Debug, Clone, Deserialize)]
struct Enemies {
    pub enemies: Vec<Enemy>,
}

// 敵の集まりの実装。
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

// 敵の定義。HPなどを持つ
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Enemy {
    name: String,
    level: Level,
    hp: f32,
}

// プレイヤーの定義。HPなどを持つ
struct Player {
    hp: f32,
    skills: Skills,
    item_containers: Vec<ItemContainer>,
}

// 敵の日本語表現。名前を表示する。
impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// 主処理のメッセージ
#[derive(Debug, Clone)]
enum Message {
    Next,
    Info(String),
    UpdateSelectorAndInfo(RandomCollection, String),
    ShowItemsForPick,
    HideItemsForPick,
    WaitingSelectItemByUser(Item),
    GiveSelectedItemForUser,
    RandomEncounter(RandomCollection),
    BattleOperationMenu(battle_operation_menu::Message),
    UseSkillMenu(use_skill_menu::Message),
    TargetEnemyMenu(target_enemy_menu::Message),
}

// 戦闘操作の列挙型
#[derive(Clone, Debug, PartialEq)]
enum BattleOperation {
    ShowSkills,
    ShowItemStocks,
}

// 戦闘操作の日本語を定義
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
        let targetable_enemies = master_data.enemies.random_pick( Level { value: 1 }, 2);

        // 初期化
        let first_message: String = "おうさま：おお　ゆうしゃよ　まおうを　たおしに　ゆくのじゃ".into();
        let usable_skills = Skills { skills: master_data.skills.random_pick( Rarity { value: 1 }, 2) };

        Self {
            battle_operation_menu: None,
            use_skill_menu: None,
            target_enemy_menu: None,
            encountered_enemies_info: vec![],
            //データ
            scenario: vec![
                // Message::Info(first_message.clone()),
                // Message::Info("おうさま：アイテムを　ひとつ　さずけよう。".into()),
                // Message::UpdateSelectorAndInfo(RandomItemCollection(Rarity::new(1), 2), "どの　アイテムを　もらう？".into()),
                // Message::ShowItemsForPick,
                // Message::GiveSelectedItemForUser,
                Message::Info("さあ　まおうを　たおす　たびの　はじまりだ。".into()),
                Message::RandomEncounter(RandomEnemyCollection(Level::new(1), 3)),
                Message::BattleOperationMenu(battle_operation_menu::Message::Initial),
            ],
            scenario_idx: 0,
            master_data,
            system_info: first_message.clone(),
            items_for_get: vec![],
            owned_items: vec![],
            selected_item: None,
            encountered_enemies: None,
            usable_skills,
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
                        self.encountered_enemies = Some(Enemies { enemies: self.master_data.enemies.random_pick(level, count as usize) })  ;
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
            Message::RandomEncounter(random_enemy_collection) => {
                match random_enemy_collection {
                    RandomEnemyCollection(level, count) => {
                        self.system_info = "てきが　あらわれた！".to_string();
                        let enemies = self.master_data.enemies.random_pick(level, count as usize);
                        self.encountered_enemies = Some(Enemies { enemies });
                        self.show_next_button = true;
                    }

                    _ => {
                        panic!("random_enemy_collection is not Enemy");
                    }
                }
                self.encountered_enemies_info = vec![];
                if let Some(enemies) = self.encountered_enemies.clone() {
                    for enemy in enemies.enemies.iter() {
                        self.encountered_enemies_info.push(format!("{}", enemy.name.as_str()));
                    }

                }

            }
            Message::BattleOperationMenu(message) => {
                self.battle_operation_menu = Some(battle_operation_menu::BattleOperationMenu::new());
                // 戦闘操作メニューを表示する
                // NOTE : match の後に update 呼び出しが必要
                match &message {
                    battle_operation_menu::Message::Initial => {
                        let mut menu = battle_operation_menu::BattleOperationMenu::new();
                        self.battle_operation_menu = Some(menu);
                    }
                    battle_operation_menu::Message::OnSelectOperation(operation) => {
                        // TODO : 不要なら削除
                        let mut menu = battle_operation_menu::BattleOperationMenu::new();
                        menu.operation = Some(operation.clone());

                        self.battle_operation_menu = Some(menu);
                    }
                    battle_operation_menu::Message::OnClickNext => {
                        self.battle_operation_menu = None;
                        self.use_skill_menu = Some(use_skill_menu::UseSkillMenu::new( self.usable_skills.clone()  ));
                    }
                }
                match self.battle_operation_menu.clone() {
                    None => {}
                    Some(mut menu) => {
                        menu.update(message.clone());
                    }
                }
            }
            Message::UseSkillMenu(message) => {
                // スキル選択メニュー
                match message {
                    use_skill_menu::Message::Initial => {
                        // スキルメニューを表示する
                        self.use_skill_menu = Some(use_skill_menu::UseSkillMenu::new(
                            self.usable_skills.clone(),
                        ))
                    }
                    use_skill_menu::Message::OnSelectSkill(ref skill) => {
                        // 何もしない
                    }
                    use_skill_menu::Message::OnClickNext => {
                        match &self.encountered_enemies {
                            None => {
                                panic!("self.encountered_enemies is None");
                            }
                            Some(encountered_enemies) => {
                                self.target_enemy_menu = Some(target_enemy_menu::TargetEnemyMenu::new(encountered_enemies.clone()));
                            }
                        }
                        self.use_skill_menu = None;
                    }
                    use_skill_menu::Message::OnClickBack => {
                        // 戦闘操作メニューを表示する
                        self.use_skill_menu = None;
                        self.battle_operation_menu = Some(
                            battle_operation_menu::BattleOperationMenu::new()
                        );
                        // スキルメニューを非表示にする
                        self.use_skill_menu = None;
                    }
                }
                match &mut self.use_skill_menu {
                    None => {}
                    Some(menu) => {
                        menu.update(message);
                    }
                }
            },
            Message::TargetEnemyMenu(message) => {

                match message {
                    target_enemy_menu::Message::Initial => {}
                    target_enemy_menu::Message::OnSelectEnemy(_) => {
                        if let Some(menu) = &mut self.target_enemy_menu {
                            menu.enemy = None;
                        }
                    }
                    target_enemy_menu::Message::OnClickNext => {
                    }
                    target_enemy_menu::Message::OnClickBack => {
                        self.target_enemy_menu = None;
                        self.use_skill_menu = Some(use_skill_menu::UseSkillMenu::new(self.usable_skills.clone()));
                    }
                }
                if let Some(menu) = &mut self.target_enemy_menu {
                    menu.update(message);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new();
        let system_info = Text::new(self.system_info.as_str());
        column = column.push(system_info);
        for enemy_info  in self.encountered_enemies_info.iter() {
            column = column.push(Text::new(enemy_info));
        }

        // サブビューの表示
        if let Some(menu) = &self.battle_operation_menu {
            column = column.push(menu.view().map(|message|Message::BattleOperationMenu(message)));
        }
        if let Some(menu) = &self.use_skill_menu {
            column = column.push(menu.view().map(|message|Message::UseSkillMenu(message)));
        }
        if let Some(menu) = &self.target_enemy_menu {
            column = column.push(menu.view().map(|message|Message::TargetEnemyMenu(message)));
        }

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
