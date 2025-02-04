// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_sci_string::register(runner);
    from_string::register(runner);
    to_sci::register(runner);
    to_string::register(runner);
}

mod from_sci_string;
mod from_string;
mod to_sci;
mod to_string;
