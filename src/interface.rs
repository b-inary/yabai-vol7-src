/// アクションを表す型
pub type Action = usize;

/// パブリックな履歴を表す型
pub type PublicHistory = Vec<Action>;

/// ゲームの定義を表すインターフェース
pub trait Game {
    /// ゲーム木のノードを表す型
    type Node: GameNode;

    /// ゲーム木の根、すなわちゲームの初期履歴を返す
    fn root() -> Self::Node;

    /// プライベートな手札の組み合わせの個数を返す
    fn num_private_hands() -> usize;

    /// 終端履歴 `node` において、最初の偶然手番の寄与を含まない counterfactual-到達確率が
    /// `pmi` のときの `player` の counterfactual value を計算する
    fn evaluate(&self, node: &Self::Node, player: usize, pmi: &Vec<f64>) -> Vec<f64>;
}

/// ゲーム木のノードを表すインターフェース
pub trait GameNode {
    /// 現在のパブリックな履歴を返す
    fn public_history(&self) -> &PublicHistory;

    /// 現在のノードが終端履歴かどうかを返す
    fn is_terminal(&self) -> bool;

    /// 現在の手番のプレイヤーを返す
    fn current_player(&self) -> usize;

    /// 着手可能なアクションの個数を返す
    fn num_actions(&self) -> Action;

    /// 着手可能なアクションの一覧を返す
    fn actions(&self) -> std::ops::Range<Action> {
        0..self.num_actions()
    }

    /// `action` を行った後のノードを返す
    fn play(&self, action: Action) -> Self;
}
