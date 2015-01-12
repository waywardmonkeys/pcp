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

use event::VarEvent;

#[derive(Copy, PartialEq, Eq, Show)]
pub enum Status
{
  Entailed,
  Disentailed,
  Unknown
}

pub trait Propagator
{
  type Event: VarEvent;

  // If the result is Entailed or Disentailed, it must not
  // change after a propagate call.
  fn status(&self) -> Status;

  // The propagator is stable when all Event are "Nothing".

  // Should Nothing exist? Or should the vector be empty in this case?...
  fn propagate(&mut self) -> Vec<(u32, Self::Event)>;
}
