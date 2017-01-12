// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gcollections::*;
use gcollections::ops::*;
use kernel::*;
use model::*;
use term::ops::*;
use variable::ops::*;
use propagation::events::*;
use propagation::concept::*;
use propagation::ops::*;
use interval::ops::Range;
use num::{Signed, Integer};
use std::ops::*;
use std::fmt::Debug;

pub use variable::concept::*;

pub trait IntBound:
  Integer + Clone + Debug
  + Signed // Due to the lack of Subtraction in term/
{}

impl<R> IntBound for R where
  R: Integer + Clone + Debug
  + Signed
{}

pub trait IntDomain:
  Bounded + Cardinality + Empty + IsEmpty + Singleton + IsSingleton + Range + Contains +
  ShrinkLeft + ShrinkRight + StrictShrinkLeft + StrictShrinkRight +
  Difference<<Self as Collection>::Item, Output=Self> +
  Intersection<Output=Self> + Difference<Output=Self> + Overlap + Subset + Disjoint +
  Add<<Self as Collection>::Item, Output=Self> + Sub<<Self as Collection>::Item, Output=Self> +
  Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> +
  Clone + Debug
where
  <Self as Collection>::Item: IntBound
{}

impl<R> IntDomain for R where
  R: Bounded + Cardinality + Empty + IsEmpty + Singleton + IsSingleton + Range + Contains,
  R: ShrinkLeft + ShrinkRight + StrictShrinkLeft + StrictShrinkRight,
  R: Difference<<R as Collection>::Item, Output=R>,
  R: Intersection<Output=R> + Difference<Output=R> + Overlap + Subset + Disjoint,
  R: Add<<R as Collection>::Item, Output=R> + Sub<<R as Collection>::Item, Output=R>,
  R: Add<Output=R> + Sub<Output=R> + Mul<Output=R>,
  R: Clone + Debug,
  <R as Collection>::Item: IntBound
{}

pub trait IntVariable_<VStore>:
  ViewDependencies<FDEvent> +
  StoreMonotonicUpdate<VStore> +
  StoreRead<VStore> +
  Debug + DisplayStateful<Model>
 where VStore: Collection
{}

impl<R, VStore> IntVariable_<VStore> for R where
  R: ViewDependencies<FDEvent>,
  R: StoreMonotonicUpdate<VStore>,
  R: StoreRead<VStore>,
  R: Debug + DisplayStateful<Model>,
  VStore: Collection
{}

pub type Var<VStore> = Box<IntVariable<VStore>>;

pub trait IntVariable<VStore>: IntVariable_<VStore>
  where VStore: Collection
{
  fn bclone(&self) -> Box<IntVariable<VStore>>;
}

impl<VStore, R> IntVariable<VStore> for R where
  R: IntVariable_<VStore>,
  R: Clone + 'static,
  VStore: Collection
{
  fn bclone(&self) -> Box<IntVariable<VStore>> {
    Box::new(self.clone())
  }
}

pub trait IntCStore<VStore>:
  Alloc + Empty + Clone + Freeze + DisplayStateful<Model> + DisplayStateful<(Model, VStore)> +
  Collection<Item=Box<PropagatorConcept<VStore, FDEvent>>> +
  Consistency<VStore> +
  PropagatorConcept<VStore, FDEvent> + Propagator<VStore>
{}

impl<R, VStore> IntCStore<VStore> for R where
  R: Alloc + Empty + Clone + Freeze + DisplayStateful<Model> + DisplayStateful<(Model, VStore)>,
  R: Collection<Item=Box<PropagatorConcept<VStore, FDEvent>>>,
  R: Consistency<VStore>,
  R: PropagatorConcept<VStore, FDEvent> + Propagator<VStore>
{}