// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The kernel is a set of reusable traits shared among the different modules. It does not provide specific implementations.

pub mod trilean;
pub mod consistency;
pub mod merge;
pub mod event;
pub mod restoration;
pub mod display_stateful;

pub use kernel::trilean::*;
pub use kernel::consistency::*;
pub use kernel::merge::*;
pub use kernel::event::*;
pub use kernel::restoration::*;
pub use kernel::display_stateful::*;
