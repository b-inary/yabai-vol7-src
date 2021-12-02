use crate::interface::*;
use bincode::deserialize;
use once_cell::sync::Lazy;
use std::{fs::File, io::Read};

const FOLD: usize = 0;
const PUSH_CALL: usize = 1;

static WIN_FREQ_TABLE: Lazy<Vec<i32>> = Lazy::new(|| {
    let path = "static/headsup_preflop_equity.bin";
    let mut infile = File::open(path).expect(&format!("could not open '{}'", path));
    let mut buf = Vec::new();
    infile.read_to_end(&mut buf).unwrap();
    deserialize::<Vec<i32>>(&buf).unwrap()
});

pub struct PushFoldGame {
    effective_stack: f64,
}

#[derive(Clone)]
pub struct PushFoldNode {
    public_history: PublicHistory,
}

impl Game for PushFoldGame {
    type Node = PushFoldNode;

    #[inline]
    fn root() -> PushFoldNode {
        PushFoldNode {
            public_history: Vec::new(),
        }
    }

    #[inline]
    fn num_private_hands() -> usize {
        52 * 51 / 2
    }

    #[inline]
    fn evaluate(&self, node: &PushFoldNode, player: usize, pmi: &Vec<f64>) -> Vec<f64> {
        let num_hands_inv = (2. * 2.) / (52. * 51. * 50. * 49.);
        let num_board = (48 * 47 * 46 * 45 * 44) / (5 * 4 * 3 * 2);
        let num_board_inv = 1.0 / num_board as f64;

        // どちらかのプレイヤーがフォールド
        if node.public_history.last() == Some(&FOLD) {
            let pmi_sum = pmi.iter().sum::<f64>();
            let mut pmi_sum_ex = [0.0; 52];

            let mut k = 0;
            for i in 0..51 {
                for j in (i + 1)..52 {
                    pmi_sum_ex[i] += pmi[k];
                    pmi_sum_ex[j] += pmi[k];
                    k += 1;
                }
            }

            let payoff = match node.public_history.len() {
                1 => [-0.5, 0.5][player],
                _ => [1.0, -1.0][player],
            } * num_hands_inv;

            let mut k = 0;
            let mut ret = Vec::with_capacity(Self::num_private_hands());
            for i in 0..51 {
                for j in (i + 1)..52 {
                    // 包除原理
                    ret.push(payoff * (pmi_sum - pmi_sum_ex[i] - pmi_sum_ex[j] + pmi[k]));
                    k += 1;
                }
            }

            return ret;
        }

        // ショーダウン
        let mut k = 0;
        let mut ret = Vec::with_capacity(Self::num_private_hands());
        for i in 0..51 {
            for j in (i + 1)..52 {
                let k_start = k;
                let mut cfvalue = 0.0;
                for m in 0..51 {
                    if i == m || j == m {
                        k += 51 - m;
                        continue;
                    }
                    for n in (m + 1)..52 {
                        if i == n || j == n {
                            k += 1;
                            continue;
                        }
                        let win_freq = WIN_FREQ_TABLE[k];
                        let lose_freq = 2 * num_board - win_freq;
                        let ev = self.effective_stack * (win_freq - lose_freq) as f64;
                        cfvalue += ev * pmi[k - k_start];
                        k += 1;
                    }
                }
                ret.push(num_hands_inv * (0.5 * num_board_inv) * cfvalue);
            }
        }

        ret
    }
}

impl PushFoldGame {
    pub fn new(effective_stack: f64) -> Self {
        PushFoldGame { effective_stack }
    }
}

impl GameNode for PushFoldNode {
    #[inline]
    fn public_history(&self) -> &PublicHistory {
        &self.public_history
    }

    #[inline]
    fn is_terminal(&self) -> bool {
        match self.public_history.as_slice() {
            [] => false,
            [PUSH_CALL] => false,
            _ => true,
        }
    }

    #[inline]
    fn current_player(&self) -> usize {
        self.public_history.len() % 2
    }

    #[inline]
    fn num_actions(&self) -> usize {
        2
    }

    #[inline]
    fn play(&self, action: usize) -> Self {
        let mut ret = self.clone();
        ret.public_history.push(action);
        ret
    }
}
