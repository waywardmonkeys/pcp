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


use solver::event::*;
use interval::ncollections::ops::*;

pub trait VarIndex
{
  fn index(&self) -> usize;
}

pub trait Failure
{
  fn is_failed(&self) -> bool;
}

pub trait VarDomain: Bounded + Cardinality + Subset
{}

impl<R> VarDomain for R where
  R: Bounded + Cardinality + Subset
{}

pub trait EventUpdate<Domain: VarDomain>
{
  fn event_update<Event>(&mut self, value: Domain,
    events: &mut Vec<(usize, Event)>) -> bool
   where
    Event: MonotonicEvent<Domain>;
}

pub trait EventShrinkLeft<Domain: VarDomain>
{
  fn event_shrink_left<Event>(&mut self, lb: Domain::Bound,
    events: &mut Vec<(usize, Event)>) -> bool
   where
    Event: MonotonicEvent<Domain>;
}

pub trait EventShrinkRight<Domain: VarDomain>
{
  fn event_shrink_right<Event>(&mut self, ub: Domain::Bound,
    events: &mut Vec<(usize, Event)>) -> bool
   where
    Event: MonotonicEvent<Domain>;
}

pub trait EventRemove<Domain: VarDomain>
{
  fn event_remove<Event>(&mut self, value: Domain::Bound,
    events: &mut Vec<(usize, Event)>) -> bool
  where
    Event: MonotonicEvent<Domain>;
}

pub trait EventIntersection<Domain, RHS = Self>
{
  fn event_intersection<Event>(&mut self, other: &mut RHS,
    events: &mut Vec<(usize, Event)>) -> bool
   where
    Event: MonotonicEvent<Domain>;
}