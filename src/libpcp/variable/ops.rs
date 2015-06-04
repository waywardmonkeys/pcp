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

use interval::ncollections::ops::*;
use std::collections::vec_map::Drain;
use std::slice;

pub trait DrainDelta<Event>
{
  fn drain_delta<'a>(&'a mut self) -> Drain<'a, Event>;
}

pub trait Iterable
{
  type Value;

  fn iter<'a>(&'a self) -> slice::Iter<'a, Self::Value>;
}

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

pub trait Assign<Value>
{
  type Variable;
  fn assign(&mut self, value: Value) -> Self::Variable;
}

pub trait MonotonicUpdate<Key, Value>
{
  fn update(&mut self, key: Key, value: Value) -> bool;
}

pub trait Read<Key>
{
  type Value;
  fn read(&self, key: Key) -> Self::Value;
}

pub trait StoreMonotonicUpdate<Store, Value>
{
  fn update(&self, store: &mut Store, value: Value) -> bool;
}

pub trait StoreRead<Store>
{
  type Value;
  fn read(&self, store: &Store) -> Self::Value;
}

pub trait ViewDependencies<Event>
{
  fn dependencies(&self, event: Event) -> Vec<(usize, Event)>;
}
