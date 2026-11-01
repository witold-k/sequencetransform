// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub trait Distance<Type, DistanceType> {

    fn distance(self, other: Type) -> DistanceType;

}

pub trait RefDistance<Type, DistanceType> {

    fn rdistance(&self, other: &Type) -> DistanceType;

}

pub trait DeltaDistance<Type, DistanceType> {

    fn ddistance(self, other: Type) -> DistanceType;

}

pub trait RefDeltaDistance<Type, DistanceType> {

    fn rddistance(&self, other: &Type) -> DistanceType;

}

pub trait Delta<Type> {

    fn delta(self) -> Type;

}

pub trait RefDelta<Type> {

    fn rdelta(&self) -> Type;

}
