/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use rand::distributions::Distribution;
use rand::Rng;

// rand-op
pub struct OpCnt<R: Rng, E> {
    // Return successful or not. If false, then this execution does not count.
    op: fn(&mut R, &mut E) -> bool,
    cnt: u64,
}
impl<R: Rng, E> OpCnt<R, E> {
    pub fn new(op: fn(&mut R, &mut E) -> bool, cnt: u64) -> Self {
        Self { op, cnt }
    }
}
pub fn rand_op<'a, R: Rng, E>(
    rng: &mut R,
    env: &mut E,
    mut op_cnts: Vec<OpCnt<R, E>>,
) {
    let mut tot = 0;
    for op_cnt in op_cnts.iter() {
        tot += op_cnt.cnt;
    }
    while tot > 0 {
        let mut id = rand::distributions::Uniform::new(0, tot).sample(rng);
        for op_cnt in &mut op_cnts {
            if id < op_cnt.cnt {
                if (op_cnt.op)(rng, env) {
                    tot -= 1;
                    op_cnt.cnt -= 1;
                }
                break;
            }
            id -= op_cnt.cnt;
        }
    }
    for op_cnt in &op_cnts {
        assert_eq!(op_cnt.cnt, 0);
    }
}
