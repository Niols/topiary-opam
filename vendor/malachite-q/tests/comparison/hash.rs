// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::hash::hash;
use malachite_q::test_util::generators::rational_gen;

#[test]
fn hash_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}
