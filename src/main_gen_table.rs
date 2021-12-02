use holdem_hand_evaluator::{heads_up_win_frequency, Hand};
use rayon::prelude::*;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let empty = Hand::new();

    let result: Vec<i32> = (0..51)
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..52).into_par_iter().flat_map_iter(move |j| {
                let mut ret = Vec::with_capacity(52 * 51 / 2);
                let hand1 = Hand::from_slice(&[i, j]);
                for m in 0..51 {
                    for n in (m + 1)..52 {
                        let hand2 = Hand::from_slice(&[m, n]);
                        if (hand1.get_mask() & hand2.get_mask()) != 0 {
                            ret.push(0);
                            continue;
                        }
                        let (win, _, tie) = heads_up_win_frequency(&hand1, &hand2, &empty, &empty);
                        ret.push((2 * win + tie) as i32);
                    }
                }
                ret
            })
        })
        .collect();

    assert_eq!(result.len(), (52 * 51 / 2) * (52 * 51 / 2));
    assert_eq!(
        result.iter().map(|x| *x as u64).sum::<u64>(),
        (52 * 51 / 2) * (50 * 49 / 2) * (48 * 47 * 46 * 45 * 44 / (5 * 4 * 3 * 2))
    );

    let encoded = bincode::serialize(&result).unwrap();
    let mut outfile = File::create("static/headsup_preflop_equity.bin")?;
    outfile.write_all(&encoded)?;

    Ok(())
}
