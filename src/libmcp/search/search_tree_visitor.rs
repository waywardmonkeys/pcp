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

use solver::space::Space;
use search::branching::branch::*;

// This is inspired by the paper:
//   Search Combinators
//   Authors: Tom Schrijvers, Guido Tack, Pieter Wuille, Horst Samulowitz, Peter J. Stuckey

pub enum Status<S: Space> {
  Satisfiable,
  Unsatisfiable,
  Unknown(Vec<Branch<S>>),
  Pruned
}

pub trait SearchTreeVisitor<S: Space> {
  fn start(&mut self, _root: &S) {}
  fn enter(&mut self, current: &mut S) -> Status<S>;
}