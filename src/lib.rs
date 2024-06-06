#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]

//! # Kiddo
//!
//! A high-performance, flexible, ergonomic [k-d tree](https://en.wikipedia.org/wiki/K-d_tree) library.
//!
//! Possibly the fastest k-d tree library in the world? [See for yourself](https://sdd.github.io/kd-tree-comparison-webapp/).
//!
//! Kiddo provides:
//! - Its standard floating-point k-d tree, exposed as [`kiddo::KdTree`](`crate::KdTree`)
//! - **integer / fixed point support** via the [`Fixed`](https://docs.rs/fixed/latest/fixed/) library;
//! - **`f16` support** via the [`half`](https://docs.rs/half/latest/half/) library;
//! - **instant zero-copy deserialization** and serialization via [`Rkyv`](https://docs.rs/rkyv/latest/rkyv/) ([`Serde`](https://docs.rs/serde/latest/serde/) still available).
//! - An [`ImmutableKdTree`](`immutable::float::kdtree::ImmutableKdTree`) with space and performance advantages over the standard
//!   k-d tree, for situations where the tree does not need to be modified after creation
//!
//! Kiddo is ideal for super-fast spatial / geospatial lookups and nearest-neighbour / KNN
//! queries for low-ish numbers of dimensions, where you want to ask questions such as:
//!  - Find the [nearest_n](`float::kdtree::KdTree::nearest_n`) item(s) to a query point, ordered by distance;
//!  - Find all items [within](`float::kdtree::KdTree::within`) a specified radius of a query point;
//!  - Find the ["best" n item(s) within](`float::kdtree::KdTree::best_n_within`) a specified distance of a query point, for some definition of "best"
//!
//! ## Usage
//! ```rust
//! use nbody_tree::KdTree;
//! use nbody_tree::SquaredEuclidean;
//! use nbody_tree::NearestNeighbour;
//!
//! let entries = vec![
//!     [0f64, 0f64],
//!     [1f64, 1f64],
//!     [2f64, 2f64],
//!     [3f64, 3f64]
//! ];
//!
//! // use the nbody_tree::KdTree type to get up and running quickly with default settings
//! let mut kdtree: KdTree<_, 2> = (&entries).into();
//!
//! // How many items are in tree?
//! assert_eq!(kdtree.size(), 4);
//!
//! // find the nearest item to [0f64, 0f64].
//! // returns a tuple of (dist, index)
//! assert_eq!(
//!     kdtree.nearest_one::<SquaredEuclidean>(&[0f64, 0f64]),
//!     NearestNeighbour { distance: 0f64, item: 0 }
//! );
//!
//! // find the nearest 3 items to [0f64, 0f64], and collect into a `Vec`
//! assert_eq!(
//!     kdtree.nearest_n::<SquaredEuclidean>(&[0f64, 0f64], 3),
//!     vec![NearestNeighbour { distance: 0f64, item: 0 }, NearestNeighbour { distance: 2f64, item: 1 }, NearestNeighbour { distance: 8f64, item: 2 }]
//! );
//! ```
//!
//! See the [examples documentation](https://github.com/sdd/kiddo/tree/master/examples) for some more in-depth examples.
//! ## Optional Features

#[macro_use]
extern crate doc_comment;

pub mod best_neighbour;
#[doc(hidden)]
pub(crate) mod common;
pub mod distance_metric;
pub mod float;
mod mirror_select_nth_unstable_by;
pub mod nearest_neighbour;
#[doc(hidden)]
#[cfg(feature = "test_utils")]
pub mod test_utils;
pub mod types;

mod iter;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub mod within_unsorted_iter;


/// A floating-point k-d tree with default parameters.
///
/// `A` is the floating point type (`f32` or `f64`, or `f16` if the `f16` feature is enabled).
/// `K` is the number of dimensions. See [`KdTree`](`float::kdtree::KdTree`) for details of how to use.
///
/// To manually specify more advanced parameters, use [`KdTree`](`float::kdtree::KdTree`) directly.
/// To store positions using integer or fixed-point types, use [`fixed::kdtree::KdTree`].
pub type KdTree<A, const K: usize> = float::kdtree::KdTree<A, u64, K, 32, u32>;

pub use best_neighbour::BestNeighbour;
pub use float::distance::Manhattan;
pub use float::distance::SquaredEuclidean;
pub use nearest_neighbour::NearestNeighbour;
